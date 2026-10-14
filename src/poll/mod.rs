use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::stream::{self, StreamExt};
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn};

use crate::client::classic::extra::{
    CountryTraffic, DhcpLease, NetworkConf, PortAnomaly, PortForward, SslCertificate, Topology,
    UnasSnapshot, UpsSelector,
};
use crate::client::classic::models::{
    ClassicClient, ClassicDevice, ClientDpi, DpiEntry, HealthSubsystem, RogueAp, SpeedtestStatus,
    Sysinfo,
};
use crate::client::classic::ClassicClientHttp;
use crate::client::models::{
    ApplicationInfo, Client, Country, Device, DeviceStatistics, DpiCatalogEntry, InventoryItem,
    NamedEntity, PendingDevice, Site,
};
use crate::client::paths::Endpoint;
use crate::client::ReadOnlyClient;
use crate::metrics::Metrics;

#[derive(Debug, Clone, Default)]
pub struct SiteSnapshot {
    pub site: Site,
    pub devices: Vec<DeviceWithStats>,
    pub clients: Vec<Client>,
    pub classic_clients: Vec<ClassicClient>,
    pub classic_devices: Vec<ClassicDevice>,
    pub health: Vec<HealthSubsystem>,
    pub dpi: Vec<DpiEntry>,
    pub client_dpi: Vec<ClientDpi>,
    pub sysinfo: Option<Sysinfo>,
    pub rogue_aps: Vec<RogueAp>,
    pub speedtest: Option<SpeedtestStatus>,
    pub networks: Vec<NamedEntity>,
    pub wifi_broadcasts: Vec<NamedEntity>,
    pub wans: Vec<NamedEntity>,
    pub vpn_servers: Vec<NamedEntity>,
    pub vpn_tunnels: Vec<NamedEntity>,
    pub device_tags: Vec<NamedEntity>,
    pub radius_profiles: Vec<NamedEntity>,
    pub firewall_policies: Vec<InventoryItem>,
    pub firewall_zones: Vec<NamedEntity>,
    pub acl_rules: Vec<InventoryItem>,
    pub dns_policies: Vec<InventoryItem>,
    pub traffic_matching_lists: Vec<NamedEntity>,
    pub hotspot_vouchers: Vec<InventoryItem>,
    pub lags: Vec<NamedEntity>,
    pub mc_lag_domains: Vec<NamedEntity>,
    pub switch_stacks: Vec<NamedEntity>,
    pub dhcp_leases: Vec<DhcpLease>,
    pub networkconf: Vec<NetworkConf>,
    pub port_forwards: Vec<PortForward>,
    pub ssl_certs: Vec<SslCertificate>,
    pub topology: Option<Topology>,
    pub country_traffic: Vec<CountryTraffic>,
    pub ups_devices: Vec<UpsSelector>,
    pub port_anomalies: Vec<PortAnomaly>,
}

#[derive(Debug, Clone)]
pub struct DeviceWithStats {
    pub device: Device,
    pub stats: Option<DeviceStatistics>,
}

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub info: ApplicationInfo,
    pub pending_devices: Vec<PendingDevice>,
    pub sites: Vec<SiteSnapshot>,
    pub fetched_at: Option<Instant>,
    pub last_error: Option<String>,
    pub last_poll_duration: Option<Duration>,
    pub poll_ok: bool,
    pub integration_ok: bool,
    pub classic_ok: bool,
    pub source: String,
    pub countries: Vec<Country>,
    pub dpi_applications: Vec<DpiCatalogEntry>,
    pub dpi_categories: Vec<DpiCatalogEntry>,
    pub unas: Option<UnasSnapshot>,
}

#[derive(Clone)]
pub struct SnapshotCache {
    inner: Arc<RwLock<Snapshot>>,
}

impl SnapshotCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Snapshot::default())),
        }
    }

    pub async fn get(&self) -> Snapshot {
        self.inner.read().await.clone()
    }

    pub async fn store_success(&self, mut snap: Snapshot, duration: Duration) {
        snap.fetched_at = Some(Instant::now());
        snap.last_poll_duration = Some(duration);
        snap.poll_ok = snap.integration_ok || snap.classic_ok;
        *self.inner.write().await = snap;
    }

    pub async fn store_failure(&self, err: String, duration: Duration) {
        let mut guard = self.inner.write().await;
        guard.last_error = Some(err);
        guard.last_poll_duration = Some(duration);
        guard.poll_ok = false;
        guard.integration_ok = false;
        guard.classic_ok = false;
    }
}

pub async fn run_poller(
    integration: Option<ReadOnlyClient>,
    classic: Option<ClassicClientHttp>,
    default_site: String,
    source: String,
    cache: SnapshotCache,
    metrics: Arc<Metrics>,
    interval: Duration,
    max_concurrency: usize,
) {
    let integration = integration.map(Arc::new);
    let classic = classic.map(Arc::new);
    loop {
        let started = Instant::now();
        match collect_snapshot(
            integration.clone(),
            classic.clone(),
            &default_site,
            &source,
            max_concurrency,
        )
        .await
        {
            Ok(snap) => {
                let dur = started.elapsed();
                info!(
                    sites = snap.sites.len(),
                    integration_ok = snap.integration_ok,
                    classic_ok = snap.classic_ok,
                    ?dur,
                    "poll succeeded"
                );
                cache.store_success(snap, dur).await;
            }
            Err(err) => {
                let dur = started.elapsed();
                warn!(error = %err, ?dur, "poll failed; keeping last snapshot");
                metrics.note_poll_error();
                cache.store_failure(err.to_string(), dur).await;
            }
        }
        tokio::time::sleep(interval).await;
    }
}

pub async fn collect_snapshot(
    integration: Option<Arc<ReadOnlyClient>>,
    classic: Option<Arc<ClassicClientHttp>>,
    default_site: &str,
    source: &str,
    max_concurrency: usize,
) -> anyhow::Result<Snapshot> {
    let mut errors: Vec<String> = Vec::new();

    let mut info = ApplicationInfo::default();
    let mut pending_devices = Vec::new();
    let mut sites: Vec<SiteSnapshot> = Vec::new();
    let mut integration_ok = false;
    let mut countries = Vec::new();
    let mut dpi_applications = Vec::new();
    let mut dpi_categories = Vec::new();

    if let Some(client) = integration {
        match collect_integration(client, max_concurrency).await {
            Ok((i, pending, s, cats)) => {
                info = i;
                pending_devices = pending;
                sites = s;
                countries = cats.0;
                dpi_applications = cats.1;
                dpi_categories = cats.2;
                integration_ok = true;
            }
            Err(e) => {
                warn!(error = %e, "integration poll failed");
                errors.push(format!("integration: {e}"));
            }
        }
    }

    let mut classic_ok = false;
    let mut unas = None;
    if let Some(client) = classic {
        match attach_classic(client, &mut sites, default_site).await {
            Ok(u) => {
                classic_ok = true;
                unas = u;
            }
            Err(e) => {
                warn!(error = %e, "classic poll failed");
                errors.push(format!("classic: {e}"));
            }
        }
    }

    if !integration_ok && !classic_ok {
        anyhow::bail!(errors.join("; "));
    }

    Ok(Snapshot {
        info,
        pending_devices,
        sites,
        fetched_at: None,
        last_error: if errors.is_empty() {
            None
        } else {
            Some(errors.join("; "))
        },
        last_poll_duration: None,
        poll_ok: true,
        integration_ok,
        classic_ok,
        source: source.to_string(),
        countries,
        dpi_applications,
        dpi_categories,
        unas,
    })
}

async fn collect_integration(
    client: Arc<ReadOnlyClient>,
    max_concurrency: usize,
) -> anyhow::Result<(
    ApplicationInfo,
    Vec<PendingDevice>,
    Vec<SiteSnapshot>,
    (
        Vec<Country>,
        Vec<DpiCatalogEntry>,
        Vec<DpiCatalogEntry>,
    ),
)> {
    let info = client.info().await?;
    let sites = client.sites().await?;
    let pending_devices = client.pending_devices().await.unwrap_or_default();
    let countries = client.countries().await.unwrap_or_default();
    let dpi_applications = client.dpi_applications().await.unwrap_or_default();
    let dpi_categories = client.dpi_categories().await.unwrap_or_default();

    let site_snaps = Arc::new(Mutex::new(Vec::with_capacity(sites.len())));

    stream::iter(sites.into_iter())
        .for_each_concurrent(max_concurrency, |site| {
            let client = client.clone();
            let site_snaps = site_snaps.clone();
            async move {
                match collect_integration_site(client, site, max_concurrency).await {
                    Ok(s) => site_snaps.lock().await.push(s),
                    Err(e) => warn!(error = %e, "site collect failed"),
                }
            }
        })
        .await;

    let sites = site_snaps.lock().await.drain(..).collect();
    Ok((
        info,
        pending_devices,
        sites,
        (countries, dpi_applications, dpi_categories),
    ))
}

async fn collect_integration_site(
    client: Arc<ReadOnlyClient>,
    site: Site,
    max_concurrency: usize,
) -> anyhow::Result<SiteSnapshot> {
    let site_id = site.id.clone();

    let (
        devices,
        clients,
        networks,
        wifi_broadcasts,
        wans,
        vpn_servers,
        vpn_tunnels,
        device_tags,
        radius_profiles,
        firewall_policies,
        firewall_zones,
        acl_rules,
        dns_policies,
        traffic_matching_lists,
        hotspot_vouchers,
        lags,
        mc_lag_domains,
        switch_stacks,
    ) = tokio::join!(
        client.devices(&site_id),
        client.clients(&site_id),
        client.named_collection(Endpoint::Networks, &site_id),
        client.named_collection(Endpoint::WifiBroadcasts, &site_id),
        client.named_collection(Endpoint::Wans, &site_id),
        client.named_collection(Endpoint::VpnServers, &site_id),
        client.named_collection(Endpoint::VpnSiteToSiteTunnels, &site_id),
        client.named_collection(Endpoint::DeviceTags, &site_id),
        client.named_collection(Endpoint::RadiusProfiles, &site_id),
        client.inventory_collection(Endpoint::FirewallPolicies, &site_id),
        client.named_collection(Endpoint::FirewallZones, &site_id),
        client.inventory_collection(Endpoint::AclRules, &site_id),
        client.inventory_collection(Endpoint::DnsPolicies, &site_id),
        client.named_collection(Endpoint::TrafficMatchingLists, &site_id),
        client.inventory_collection(Endpoint::HotspotVouchers, &site_id),
        client.named_collection(Endpoint::SwitchingLags, &site_id),
        client.named_collection(Endpoint::SwitchingMcLagDomains, &site_id),
        client.named_collection(Endpoint::SwitchingSwitchStacks, &site_id),
    );

    let devices = devices.unwrap_or_default();
    let devices_with_stats =
        fetch_device_stats(client.clone(), &site_id, devices, max_concurrency).await;

    Ok(SiteSnapshot {
        site,
        devices: devices_with_stats,
        clients: clients.unwrap_or_default(),
        networks: networks.unwrap_or_default(),
        wifi_broadcasts: wifi_broadcasts.unwrap_or_default(),
        wans: wans.unwrap_or_default(),
        vpn_servers: vpn_servers.unwrap_or_default(),
        vpn_tunnels: vpn_tunnels.unwrap_or_default(),
        device_tags: device_tags.unwrap_or_default(),
        radius_profiles: radius_profiles.unwrap_or_default(),
        firewall_policies: firewall_policies.unwrap_or_default(),
        firewall_zones: firewall_zones.unwrap_or_default(),
        acl_rules: acl_rules.unwrap_or_default(),
        dns_policies: dns_policies.unwrap_or_default(),
        traffic_matching_lists: traffic_matching_lists.unwrap_or_default(),
        hotspot_vouchers: hotspot_vouchers.unwrap_or_default(),
        lags: lags.unwrap_or_default(),
        mc_lag_domains: mc_lag_domains.unwrap_or_default(),
        switch_stacks: switch_stacks.unwrap_or_default(),
        ..Default::default()
    })
}

async fn attach_classic(
    client: Arc<ClassicClientHttp>,
    sites: &mut Vec<SiteSnapshot>,
    default_site: &str,
) -> anyhow::Result<Option<UnasSnapshot>> {
    if sites.is_empty() {
        sites.push(SiteSnapshot {
            site: Site {
                id: default_site.to_string(),
                name: Some(default_site.to_string()),
                internal_reference: Some(default_site.to_string()),
            },
            ..Default::default()
        });
    }

    for site in sites.iter_mut() {
        let classic_name = site
            .site
            .internal_reference
            .as_deref()
            .filter(|s| !s.is_empty())
            .or(site.site.name.as_deref())
            .unwrap_or(default_site);

        let (clients, devices, health, dpi, rogue, speedtest, sysinfo, stadpi) = tokio::join!(
            client.clients(classic_name),
            client.devices(classic_name),
            client.health(classic_name),
            client.dpi(classic_name),
            client.rogue_aps(classic_name),
            client.speedtest_status(classic_name),
            client.sysinfo(classic_name),
            client.client_dpi(classic_name),
        );
        let (leases, networks, forwards, certs, topology, country, ups, anomalies) = tokio::join!(
            client.dhcp_leases(classic_name),
            client.networkconf(classic_name),
            client.port_forwards(classic_name),
            client.ssl_certs(classic_name),
            client.topology(classic_name),
            client.country_traffic(classic_name),
            client.ups_devices(classic_name),
            client.port_anomalies(classic_name),
        );

        site.classic_clients = clients.map_err(|e| anyhow::anyhow!("stat/sta: {e}"))?;
        site.classic_devices = devices.map_err(|e| anyhow::anyhow!("stat/device: {e}"))?;
        site.health = health.unwrap_or_default();
        site.dpi = dpi.unwrap_or_default();
        site.rogue_aps = rogue.unwrap_or_default();
        site.speedtest = speedtest.unwrap_or_default();
        site.sysinfo = sysinfo.unwrap_or_default();
        site.client_dpi = stadpi.unwrap_or_default();
        site.dhcp_leases = leases.unwrap_or_default();
        site.networkconf = networks.unwrap_or_default();
        site.port_forwards = forwards.unwrap_or_default();
        site.ssl_certs = certs.unwrap_or_default();
        site.topology = topology.unwrap_or_default();
        site.country_traffic = country.unwrap_or_default();
        site.ups_devices = ups.unwrap_or_default();
        site.port_anomalies = anomalies.unwrap_or_default();
    }
    let unas = client.unas().await.unwrap_or_default();
    Ok(unas)
}

async fn fetch_device_stats(
    client: Arc<ReadOnlyClient>,
    site_id: &str,
    devices: Vec<Device>,
    max_concurrency: usize,
) -> Vec<DeviceWithStats> {
    let results = Arc::new(Mutex::new(HashMap::new()));
    stream::iter(devices.into_iter())
        .for_each_concurrent(max_concurrency, |device| {
            let client = client.clone();
            let results = results.clone();
            let site_id = site_id.to_string();
            async move {
                let stats = match client.device_statistics(&site_id, &device.id).await {
                    Ok(s) => Some(s),
                    Err(e) => {
                        warn!(device = %device.id, error = %e, "device statistics unavailable");
                        None
                    }
                };
                results
                    .lock()
                    .await
                    .insert(device.id.clone(), DeviceWithStats { device, stats });
            }
        })
        .await;

    let mut guard = results.lock().await;
    guard.drain().map(|(_, v)| v).collect()
}
