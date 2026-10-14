use prometheus::{
    Encoder, Gauge, IntCounter, IntGauge, IntGaugeVec, Opts, Registry, TextEncoder,
};

use crate::poll::Snapshot;

mod classic_extra;
mod clients;
mod common;
mod devices;
mod extras;
mod integration;
mod inventory;
mod ports;
mod radios;

pub struct Metrics {
    pub registry: Registry,
    // exporter health
    up: IntGauge,
    scrape_success: IntGauge,
    cache_age_seconds: Gauge,
    poll_duration_seconds: Gauge,
    poll_errors_total: IntCounter,
    integration_up: IntGauge,
    classic_up: IntGauge,
    application_info: IntGaugeVec,
    site_info: IntGaugeVec,
    site_count: IntGauge,
    pending_device_count: IntGauge,
    pending_device_info: IntGaugeVec,
    devices: devices::DeviceMetrics,
    clients: clients::ClientMetrics,
    inventory: inventory::InventoryMetrics,
    extras: extras::ExtrasMetrics,
    ports: ports::PortMetrics,
    radios: radios::RadioMetrics,
    classic_extra: classic_extra::ClassicExtraMetrics,
    integration: integration::IntegrationMetrics,
}

impl Metrics {
    pub fn new() -> anyhow::Result<Self> {
        let registry = Registry::new();

        let up = IntGauge::with_opts(Opts::new(
            "unifi_exporter_up",
            "1 if the exporter process is running",
       ))?;
        let scrape_success = IntGauge::with_opts(Opts::new(
            "unifi_exporter_scrape_success",
            "1 if the last background poll succeeded",
        ))?;
        let cache_age_seconds = Gauge::with_opts(Opts::new(
            "unifi_exporter_cache_age_seconds",
            "Seconds since the last successful snapshot refresh (-1 if never)",
        ))?;
        let poll_duration_seconds = Gauge::with_opts(Opts::new(
            "unifi_exporter_poll_duration_seconds",
            "Duration of the last poll attempt in seconds",
        ))?;
        let poll_errors_total = IntCounter::with_opts(Opts::new(
            "unifi_exporter_poll_errors_total",
            "Total number of failed background polls",
        ))?;
        let integration_up = IntGauge::with_opts(Opts::new(
            "unifi_exporter_integration_up",
            "1 if the last Integration API poll succeeded",
        ))?;
        let classic_up = IntGauge::with_opts(Opts::new(
            "unifi_exporter_classic_up",
            "1 if the last classic controller poll succeeded",
        ))?;

        let application_info = IntGaugeVec::new(
            Opts::new(
                "unifi_application_info",
                "UniFi Network application info (value always 1)",
            ),
            &["version"],
        )?;
        let site_info = IntGaugeVec::new(
            Opts::new("unifi_site_info", "UniFi site info (value always 1)"),
            &["site_id", "site_name", "internal_reference"],
        )?;
        let site_count =
            IntGauge::with_opts(Opts::new("unifi_sites", "Number of UniFi sites"))?;
        let pending_device_count = IntGauge::with_opts(Opts::new(
            "unifi_pending_devices",
            "Number of devices pending adoption",
        ))?;
        let pending_device_info = IntGaugeVec::new(
            Opts::new(
                "unifi_pending_device_info",
                "Pending device identity (value always 1)",
            ),
            &["id", "name", "model", "mac"],
        )?;

        registry.register(Box::new(up.clone()))?;
        registry.register(Box::new(scrape_success.clone()))?;
        registry.register(Box::new(cache_age_seconds.clone()))?;
        registry.register(Box::new(poll_duration_seconds.clone()))?;
        registry.register(Box::new(poll_errors_total.clone()))?;
        registry.register(Box::new(integration_up.clone()))?;
        registry.register(Box::new(classic_up.clone()))?;
        registry.register(Box::new(application_info.clone()))?;
        registry.register(Box::new(site_info.clone()))?;
        registry.register(Box::new(site_count.clone()))?;
        registry.register(Box::new(pending_device_count.clone()))?;
        registry.register(Box::new(pending_device_info.clone()))?;

        let devices = devices::DeviceMetrics::new(&registry)?;
        let clients = clients::ClientMetrics::new(&registry)?;
        let inventory = inventory::InventoryMetrics::new(&registry)?;
        let extras = extras::ExtrasMetrics::new(&registry)?;
        let ports = ports::PortMetrics::new(&registry)?;
        let radios = radios::RadioMetrics::new(&registry)?;
        let classic_extra = classic_extra::ClassicExtraMetrics::new(&registry)?;
        let integration = integration::IntegrationMetrics::new(&registry)?;

        up.set(1);

        Ok(Self {
            registry,
            up,
            scrape_success,
            cache_age_seconds,
            poll_duration_seconds,
            poll_errors_total,
            integration_up,
            classic_up,
            application_info,
            site_info,
            site_count,
            pending_device_count,
            pending_device_info,
            devices,
            clients,
            inventory,
            extras,
            ports,
            radios,
            classic_extra,
            integration,
        })
    }

    pub fn render(&self, snap: &Snapshot) -> anyhow::Result<String> {
        self.apply(snap);
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }

    fn apply(&self, snap: &Snapshot) {
        self.up.set(1);
        self.scrape_success.set(if snap.poll_ok { 1 } else { 0 });
        self.integration_up.set(if snap.integration_ok { 1 } else { 0 });
        self.classic_up.set(if snap.classic_ok { 1 } else { 0 });

        match snap.fetched_at {
            Some(t) => self.cache_age_seconds.set(t.elapsed().as_secs_f64()),
            None => self.cache_age_seconds.set(-1.0),
        }
        if let Some(d) = snap.last_poll_duration {
            self.poll_duration_seconds.set(d.as_secs_f64());
        }

        self.application_info.reset();
        let version = snap
            .info
            .application_version
            .clone()
            .unwrap_or_else(|| "unknown".into());
        self.application_info
            .with_label_values(&[version.as_str()])
            .set(1);

        self.site_info.reset();
        self.site_count.set(snap.sites.len() as i64);
        for site in &snap.sites {
            let name = site.site.name.as_deref().unwrap_or("");
            let iref = site.site.internal_reference.as_deref().unwrap_or("");
            self.site_info
                .with_label_values(&[site.site.id.as_str(), name, iref])
                .set(1);
        }

        self.pending_device_info.reset();
        self.pending_device_count
            .set(snap.pending_devices.len() as i64);
        for d in &snap.pending_devices {
            self.pending_device_info
                .with_label_values(&[
                    d.id.as_deref().unwrap_or(""),
                    d.name.as_deref().unwrap_or(""),
                    d.model.as_deref().unwrap_or(""),
                    d.mac_address.as_deref().unwrap_or(""),
                ])
                .set(1);
        }

        self.devices.apply(snap);
        self.clients.apply(snap);
        self.inventory.apply(snap);
        self.extras.apply(snap);
        self.ports.apply(snap);
        self.radios.apply(snap);
        self.classic_extra.apply(snap);
        self.integration.apply(snap);
    }

    pub fn note_poll_error(&self) {
        self.poll_errors_total.inc();
    }
}
