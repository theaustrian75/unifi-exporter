use prometheus::{GaugeVec, IntGaugeVec, Registry};

use crate::client::classic::models::ClassicDevice;
use crate::poll::{DeviceWithStats, Snapshot};

use super::common::{gauge, igauge, set, set_i};

const DEV: &[&str] = &["device_id", "name", "type", "model", "site_name"];
const RADIO: &[&str] = &[
    "device_id",
    "name",
    "type",
    "model",
    "site_name",
    "frequency_ghz",
];
const UPLINK: &[&str] = &[
    "device_id",
    "name",
    "type",
    "model",
    "site_name",
    "uplink_index",
];

pub struct IntegrationMetrics {
    cpu: GaugeVec,
    mem: GaugeVec,
    load1: GaugeVec,
    load5: GaugeVec,
    load15: GaugeVec,
    uptime: GaugeVec,
    radio_retries: GaugeVec,
    uplink_rx: GaugeVec,
    uplink_tx: GaugeVec,
    country_present: IntGaugeVec,
    dpi_app: IntGaugeVec,
    dpi_cat: IntGaugeVec,
    pending_fw: IntGaugeVec,
    pending_supported: IntGaugeVec,
    wifi_enabled: IntGaugeVec,
    vpn_server: IntGaugeVec,
    s2s: IntGaugeVec,
    radius: IntGaugeVec,
    tml: IntGaugeVec,
    acl_enabled: IntGaugeVec,
    acl_index: GaugeVec,
    dns: IntGaugeVec,
    zone_nets: GaugeVec,
    lag_members: GaugeVec,
    mclag_lags: GaugeVec,
    mclag_peers: GaugeVec,
    stack_members: GaugeVec,
    voucher_guests: GaugeVec,
    voucher_limit: GaugeVec,
    voucher_data: GaugeVec,
    voucher_time: GaugeVec,
}

impl IntegrationMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        Ok(Self {
            cpu: gauge(
                registry,
                "unifi_integration_device_cpu_utilization_pct",
                "Device CPU % (Integration)",
                DEV,
            )?,
            mem: gauge(
                registry,
                "unifi_integration_device_memory_utilization_pct",
                "Device memory % (Integration)",
                DEV,
            )?,
            load1: gauge(
                registry,
                "unifi_integration_device_load_average_1min",
                "Device load 1m (Integration)",
                DEV,
            )?,
            load5: gauge(
                registry,
                "unifi_integration_device_load_average_5min",
                "Device load 5m (Integration)",
                DEV,
            )?,
            load15: gauge(
                registry,
                "unifi_integration_device_load_average_15min",
                "Device load 15m (Integration)",
                DEV,
            )?,
            uptime: gauge(
                registry,
                "unifi_integration_device_uptime_seconds",
                "Device uptime (Integration)",
                DEV,
            )?,
            radio_retries: gauge(
                registry,
                "unifi_integration_device_radio_tx_retries_pct",
                "Radio TX retries % (Integration)",
                RADIO,
            )?,
            uplink_rx: gauge(
                registry,
                "unifi_integration_device_uplink_rx_rate_bps",
                "Uplink RX bps (Integration)",
                UPLINK,
            )?,
            uplink_tx: gauge(
                registry,
                "unifi_integration_device_uplink_tx_rate_bps",
                "Uplink TX bps (Integration)",
                UPLINK,
            )?,
            country_present: igauge(
                registry,
                "unifi_country_present",
                "Geo-filter country catalogue entry",
                &["code", "name"],
            )?,
            dpi_app: igauge(
                registry,
                "unifi_dpi_application_present",
                "DPI application catalogue entry",
                &["app_id", "name"],
            )?,
            dpi_cat: igauge(
                registry,
                "unifi_dpi_category_present",
                "DPI category catalogue entry",
                &["cat_id", "name"],
            )?,
            pending_fw: igauge(
                registry,
                "unifi_pending_device_firmware_updatable",
                "Pending device has firmware update",
                &["mac_address", "model", "state", "firmware_version"],
            )?,
            pending_supported: igauge(
                registry,
                "unifi_pending_device_supported",
                "Pending device model is supported",
                &["mac_address", "model", "state", "firmware_version"],
            )?,
            wifi_enabled: igauge(
                registry,
                "unifi_wifi_broadcast_enabled",
                "WiFi SSID broadcast enabled",
                &["site_name", "name", "network", "security_type"],
            )?,
            vpn_server: igauge(
                registry,
                "unifi_vpn_server_enabled",
                "VPN server enabled",
                &["site_name", "name", "vpn_type", "origin"],
            )?,
            s2s: igauge(
                registry,
                "unifi_site_to_site_tunnel_present",
                "Site-to-site VPN tunnel present",
                &["site_name", "name", "tunnel_type", "origin"],
            )?,
            radius: igauge(
                registry,
                "unifi_radius_profile_present",
                "RADIUS profile present",
                &["site_name", "name", "origin"],
            )?,
            tml: igauge(
                registry,
                "unifi_traffic_matching_list_present",
                "Traffic matching list present",
                &["site_name", "name", "list_type"],
            )?,
            acl_enabled: igauge(
                registry,
                "unifi_acl_rule_enabled",
                "ACL rule enabled",
                &["site_name", "name", "action"],
            )?,
            acl_index: gauge(
                registry,
                "unifi_acl_rule_index",
                "ACL rule evaluation order",
                &["site_name", "name", "action"],
            )?,
            dns: igauge(
                registry,
                "unifi_dns_policy_enabled",
                "DNS policy enabled",
                &["site_name", "domain", "policy_type"],
            )?,
            zone_nets: gauge(
                registry,
                "unifi_firewall_zone_network_count",
                "Networks in firewall zone",
                &["site_name", "name", "origin"],
            )?,
            lag_members: gauge(
                registry,
                "unifi_lag_member_count",
                "LAG member count",
                &["site_name", "lag_id", "lag_type", "origin"],
            )?,
            mclag_lags: gauge(
                registry,
                "unifi_mclag_domain_lag_count",
                "MC-LAG domain LAG count",
                &["site_name", "name", "origin"],
            )?,
            mclag_peers: gauge(
                registry,
                "unifi_mclag_domain_peer_count",
                "MC-LAG domain peer count",
                &["site_name", "name", "origin"],
            )?,
            stack_members: gauge(
                registry,
                "unifi_switch_stack_member_count",
                "Switch stack member count",
                &["site_name", "name", "origin"],
            )?,
            voucher_guests: gauge(
                registry,
                "unifi_hotspot_voucher_authorized_guest_count",
                "Hotspot voucher authorized guests",
                &["site_name", "name", "code"],
            )?,
            voucher_limit: gauge(
                registry,
                "unifi_hotspot_voucher_authorized_guest_limit",
                "Hotspot voucher guest limit",
                &["site_name", "name", "code"],
            )?,
            voucher_data: gauge(
                registry,
                "unifi_hotspot_voucher_data_usage_limit_mbytes",
                "Hotspot voucher data cap",
                &["site_name", "name", "code"],
            )?,
            voucher_time: gauge(
                registry,
                "unifi_hotspot_voucher_time_limit_minutes",
                "Hotspot voucher time limit",
                &["site_name", "name", "code"],
            )?,
        })
    }

    pub fn apply(&self, snap: &Snapshot) {
        self.reset();
        for c in &snap.countries {
            set_i(
                &self.country_present,
                &[c.code.as_deref().unwrap_or(""), c.name.as_deref().unwrap_or("")],
                1,
            );
        }
        for a in &snap.dpi_applications {
            set_i(
                &self.dpi_app,
                &[a.id.as_deref().unwrap_or(""), a.name.as_deref().unwrap_or("")],
                1,
            );
        }
        for c in &snap.dpi_categories {
            set_i(
                &self.dpi_cat,
                &[c.id.as_deref().unwrap_or(""), c.name.as_deref().unwrap_or("")],
                1,
            );
        }
        for d in &snap.pending_devices {
            let labels = [
                d.mac_address.as_deref().unwrap_or(""),
                d.model.as_deref().unwrap_or(""),
                d.state.as_deref().unwrap_or(""),
                d.firmware_version.as_deref().unwrap_or(""),
            ];
            set_i(
                &self.pending_fw,
                &labels,
                i64::from(d.firmware_updatable.unwrap_or(false)),
            );
            set_i(
                &self.pending_supported,
                &labels,
                i64::from(d.supported.unwrap_or(false)),
            );
        }
        for site in &snap.sites {
            let site_name = site.site.name.as_deref().unwrap_or("");
            for dws in &site.devices {
                let ident = device_ident(dws, &site.classic_devices);
                if let Some(stats) = &dws.stats {
                    let labels = [
                        ident.id.as_str(),
                        ident.name.as_str(),
                        ident.dtype.as_str(),
                        ident.model.as_str(),
                        site_name,
                    ];
                    set(&self.cpu, &labels, stats.cpu_utilization_pct);
                    set(&self.mem, &labels, stats.memory_utilization_pct);
                    set(&self.load1, &labels, stats.load_average1_min);
                    set(&self.load5, &labels, stats.load_average5_min);
                    set(&self.load15, &labels, stats.load_average15_min);
                    set(&self.uptime, &labels, stats.uptime_sec.map(|v| v as f64));
                    if let Some(ifaces) = &stats.interfaces {
                        for r in &ifaces.radios {
                            let freq = r
                                .frequency_g_hz
                                .map(|f| format!("{f}"))
                                .unwrap_or_default();
                            set(
                                &self.radio_retries,
                                &[
                                    ident.id.as_str(),
                                    ident.name.as_str(),
                                    ident.dtype.as_str(),
                                    ident.model.as_str(),
                                    site_name,
                                    freq.as_str(),
                                ],
                                r.tx_retries_pct,
                            );
                        }
                    }
                    let mut uplinks = stats.uplinks.clone();
                    if uplinks.is_empty() {
                        if let Some(up) = &stats.uplink {
                            uplinks.push(up.clone());
                        }
                    }
                    for (i, up) in uplinks.iter().enumerate() {
                        let idx = i.to_string();
                        let ul = [
                            ident.id.as_str(),
                            ident.name.as_str(),
                            ident.dtype.as_str(),
                            ident.model.as_str(),
                            site_name,
                            idx.as_str(),
                        ];
                        set(&self.uplink_rx, &ul, up.rx_rate_bps.map(|v| v as f64));
                        set(&self.uplink_tx, &ul, up.tx_rate_bps.map(|v| v as f64));
                    }
                }
            }
            for wb in &site.wifi_broadcasts {
                set_i(
                    &self.wifi_enabled,
                    &[
                        site_name,
                        wb.name.as_deref().unwrap_or(&wb.id),
                        wb.network.as_deref().unwrap_or(""),
                        wb.security_type(),
                    ],
                    i64::from(wb.enabled.unwrap_or(true)),
                );
            }
            for vs in &site.vpn_servers {
                set_i(
                    &self.vpn_server,
                    &[
                        site_name,
                        vs.name.as_deref().unwrap_or(&vs.id),
                        vs.item_type.as_deref().unwrap_or(""),
                        vs.origin(),
                    ],
                    i64::from(vs.enabled.unwrap_or(true)),
                );
            }
            for t in &site.vpn_tunnels {
                set_i(
                    &self.s2s,
                    &[
                        site_name,
                        t.name.as_deref().unwrap_or(&t.id),
                        t.item_type.as_deref().unwrap_or(""),
                        t.origin(),
                    ],
                    1,
                );
            }
            for r in &site.radius_profiles {
                set_i(
                    &self.radius,
                    &[site_name, r.name.as_deref().unwrap_or(&r.id), r.origin()],
                    1,
                );
            }
            for t in &site.traffic_matching_lists {
                set_i(
                    &self.tml,
                    &[
                        site_name,
                        t.name.as_deref().unwrap_or(&t.id),
                        t.item_type.as_deref().unwrap_or(""),
                    ],
                    1,
                );
            }
            for a in &site.acl_rules {
                let labels = [
                    site_name,
                    a.name.as_deref().unwrap_or(&a.id),
                    a.action.as_deref().unwrap_or(""),
                ];
                set_i(
                    &self.acl_enabled,
                    &labels,
                    i64::from(a.enabled.unwrap_or(true)),
                );
                set(&self.acl_index, &labels, a.index);
            }
            for d in &site.dns_policies {
                set_i(
                    &self.dns,
                    &[
                        site_name,
                        d.domain.as_deref().or(d.name.as_deref()).unwrap_or(&d.id),
                        d.item_type.as_deref().unwrap_or(""),
                    ],
                    i64::from(d.enabled.unwrap_or(true)),
                );
            }
            for z in &site.firewall_zones {
                set(
                    &self.zone_nets,
                    &[site_name, z.name.as_deref().unwrap_or(&z.id), z.origin()],
                    Some(z.network_ids.len() as f64),
                );
            }
            for l in &site.lags {
                set(
                    &self.lag_members,
                    &[
                        site_name,
                        l.id.as_str(),
                        l.item_type.as_deref().unwrap_or(""),
                        l.origin(),
                    ],
                    Some(l.members.len() as f64),
                );
            }
            for d in &site.mc_lag_domains {
                let labels = [site_name, d.name.as_deref().unwrap_or(&d.id), d.origin()];
                set(&self.mclag_lags, &labels, Some(d.lags.len() as f64));
                set(&self.mclag_peers, &labels, Some(d.peers.len() as f64));
            }
            for s in &site.switch_stacks {
                set(
                    &self.stack_members,
                    &[site_name, s.name.as_deref().unwrap_or(&s.id), s.origin()],
                    Some(s.members.len() as f64),
                );
            }
            for v in &site.hotspot_vouchers {
                let labels = [
                    site_name,
                    v.name.as_deref().unwrap_or(""),
                    v.code.as_deref().unwrap_or(""),
                ];
                set(&self.voucher_guests, &labels, v.authorized_guest_count);
                set(&self.voucher_limit, &labels, v.authorized_guest_limit);
                set(&self.voucher_data, &labels, v.data_usage_limit_m_bytes);
                set(&self.voucher_time, &labels, v.time_limit_minutes);
            }
        }
    }

    fn reset(&self) {
        self.cpu.reset();
        self.mem.reset();
        self.load1.reset();
        self.load5.reset();
        self.load15.reset();
        self.uptime.reset();
        self.radio_retries.reset();
        self.uplink_rx.reset();
        self.uplink_tx.reset();
        self.country_present.reset();
        self.dpi_app.reset();
        self.dpi_cat.reset();
        self.pending_fw.reset();
        self.pending_supported.reset();
        self.wifi_enabled.reset();
        self.vpn_server.reset();
        self.s2s.reset();
        self.radius.reset();
        self.tml.reset();
        self.acl_enabled.reset();
        self.acl_index.reset();
        self.dns.reset();
        self.zone_nets.reset();
        self.lag_members.reset();
        self.mclag_lags.reset();
        self.mclag_peers.reset();
        self.stack_members.reset();
        self.voucher_guests.reset();
        self.voucher_limit.reset();
        self.voucher_data.reset();
        self.voucher_time.reset();
    }
}

struct DeviceIdent {
    id: String,
    name: String,
    dtype: String,
    model: String,
}

fn device_ident(dws: &DeviceWithStats, classic: &[ClassicDevice]) -> DeviceIdent {
    let dtype = classic
        .iter()
        .find(|c| mac_eq(c.mac.as_deref(), dws.device.mac_address.as_deref()))
        .and_then(|c| c.device_type.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| dws.device.inferred_type().to_string());
    DeviceIdent {
        id: dws.device.id.clone(),
        name: dws.device.name.clone().unwrap_or_default(),
        dtype,
        model: dws.device.model.clone().unwrap_or_default(),
    }
}

fn mac_eq(a: Option<&str>, b: Option<&str>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => mac_key(a) == mac_key(b),
        _ => false,
    }
}

fn mac_key(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
