use prometheus::{IntGaugeVec, Opts, Registry};

use crate::client::models::{InventoryItem, NamedEntity};
use crate::poll::Snapshot;

pub struct InventoryMetrics {
    entity_info: IntGaugeVec,
    entity_count: IntGaugeVec,
}

impl InventoryMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let entity_info = IntGaugeVec::new(
            Opts::new(
                "unifi_inventory_info",
                "Inventory entity from Integration API (value 1)",
            ),
            &["site_id", "site_name", "kind", "id", "name", "type", "state"],
        )?;
        let entity_count = IntGaugeVec::new(
            Opts::new(
                "unifi_inventory",
                "Count of inventory entities by kind for a site",
            ),
            &["site_id", "site_name", "kind"],
        )?;
        registry.register(Box::new(entity_info.clone()))?;
        registry.register(Box::new(entity_count.clone()))?;
        Ok(Self {
            entity_info,
            entity_count,
        })
    }

    pub fn apply(&self, snap: &Snapshot) {
        self.entity_info.reset();
        self.entity_count.reset();

        for site in &snap.sites {
            let site_id = site.site.id.as_str();
            let site_name = site.site.name.as_deref().unwrap_or("");

            emit_named(self, site_id, site_name, "network", &site.networks);
            emit_named(self, site_id, site_name, "wifi_broadcast", &site.wifi_broadcasts);
            emit_named(self, site_id, site_name, "wan", &site.wans);
            emit_named(self, site_id, site_name, "vpn_server", &site.vpn_servers);
            emit_named(self, site_id, site_name, "vpn_site_to_site_tunnel", &site.vpn_tunnels);
            emit_named(self, site_id, site_name, "device_tag", &site.device_tags);
            emit_named(self, site_id, site_name, "radius_profile", &site.radius_profiles);
            emit_named(self, site_id, site_name, "firewall_zone", &site.firewall_zones);
            emit_named(
                self,
                site_id,
                site_name,
                "traffic_matching_list",
                &site.traffic_matching_lists,
            );
            emit_named(self, site_id, site_name, "lag", &site.lags);
            emit_named(self, site_id, site_name, "mc_lag_domain", &site.mc_lag_domains);
            emit_named(self, site_id, site_name, "switch_stack", &site.switch_stacks);

            emit_inventory(self, site_id, site_name, "firewall_policy", &site.firewall_policies);
            emit_inventory(self, site_id, site_name, "acl_rule", &site.acl_rules);
            emit_inventory(self, site_id, site_name, "dns_policy", &site.dns_policies);
            emit_inventory(self, site_id, site_name, "hotspot_voucher", &site.hotspot_vouchers);
        }
    }
}

fn emit_named(
    metrics: &InventoryMetrics,
    site_id: &str,
    site_name: &str,
    kind: &str,
    items: &[NamedEntity],
) {
    metrics
        .entity_count
        .with_label_values(&[site_id, site_name, kind])
        .set(items.len() as i64);
    for item in items {
        let name = item.name.as_deref().unwrap_or("");
        metrics
            .entity_info
            .with_label_values(&[
                site_id,
                site_name,
                kind,
                &item.id,
                name,
                item.item_type.as_deref().unwrap_or(""),
                "",
            ])
            .set(1);
    }
}

fn emit_inventory(
    metrics: &InventoryMetrics,
    site_id: &str,
    site_name: &str,
    kind: &str,
    items: &[InventoryItem],
) {
    metrics
        .entity_count
        .with_label_values(&[site_id, site_name, kind])
        .set(items.len() as i64);
    for item in items {
        let name = item.name.as_deref().unwrap_or("");
        let ty = item.item_type.as_deref().unwrap_or("");
        let state = item
            .state
            .clone()
            .or_else(|| item.enabled.map(|e| if e { "enabled" } else { "disabled" }.into()))
            .unwrap_or_default();
        metrics
            .entity_info
            .with_label_values(&[
                site_id,
                site_name,
                kind,
                &item.id,
                name,
                ty,
                state.as_str(),
            ])
            .set(1);
    }
}
