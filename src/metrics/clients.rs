use prometheus::{GaugeVec, IntGauge, Opts, Registry};

use crate::client::classic::models::ClassicClient;
use crate::poll::Snapshot;

const CLIENT_LABELS: &[&str] = &[
    "name",
    "mac",
    "site_name",
    "gw_name",
    "sw_name",
    "vlan",
    "ip",
    "oui",
    "network",
    "sw_port",
    "ap_name",
    "source",
    "identity",
    "wired",
];

const WIFI_LABELS: &[&str] = &[
    "radio_name",
    "radio",
    "radio_proto",
    "channel",
    "essid",
    "bssid",
    "radio_desc",
    "name",
    "mac",
    "site_name",
    "gw_name",
    "sw_name",
    "vlan",
    "ip",
    "oui",
    "network",
    "sw_port",
    "ap_name",
    "source",
    "identity",
    "wired",
];

const DPI_LABELS: &[&str] = &["name", "mac", "site_name", "source", "category", "application"];

pub struct ClientMetrics {
    receive_bytes_total: GaugeVec,
    receive_packets_total: GaugeVec,
    receive_rate_bytes: GaugeVec,
    transmit_bytes_total: GaugeVec,
    transmit_packets_total: GaugeVec,
    transmit_rate_bytes: GaugeVec,
    transmit_retries_total: GaugeVec,
    transfer_rate_bytes: GaugeVec,
    uptime_seconds: GaugeVec,
    rssi_db: GaugeVec,
    radio_signal_db: GaugeVec,
    noise_db: GaugeVec,
    ccq_ratio: GaugeVec,
    satisfaction_ratio: GaugeVec,
    roam_count_total: GaugeVec,
    radio_receive_mcs_index: GaugeVec,
    radio_transmit_mcs_index: GaugeVec,
    radio_receive_spatial_streams: GaugeVec,
    radio_transmit_spatial_streams: GaugeVec,
    radio_receive_rate_bps: GaugeVec,
    radio_transmit_rate_bps: GaugeVec,
    radio_transmit_power_dbm: GaugeVec,
    wifi_attempts_transmit_total: GaugeVec,
    anomalies: GaugeVec,
    wired_receive_bytes_total: GaugeVec,
    wired_transmit_bytes_total: GaugeVec,
    wired_receive_packets_total: GaugeVec,
    wired_transmit_packets_total: GaugeVec,
    wired_receive_rate_bytes: GaugeVec,
    wired_transmit_rate_bytes: GaugeVec,
    dpi_transmit_packets: GaugeVec,
    dpi_receive_packets: GaugeVec,
    dpi_transmit_bytes: GaugeVec,
    dpi_receive_bytes: GaugeVec,
    count: IntGauge,
}

impl ClientMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let ns = "unifi_client_";
        let m = Self {
            receive_bytes_total: gauge(registry, &format!("{ns}receive_bytes_total"), "Client receive bytes", CLIENT_LABELS)?,
            receive_packets_total: gauge(registry, &format!("{ns}receive_packets_total"), "Client receive packets", CLIENT_LABELS)?,
            receive_rate_bytes: gauge(registry, &format!("{ns}receive_rate_bytes"), "Client receive rate", CLIENT_LABELS)?,
            transmit_bytes_total: gauge(registry, &format!("{ns}transmit_bytes_total"), "Client transmit bytes", CLIENT_LABELS)?,
            transmit_packets_total: gauge(registry, &format!("{ns}transmit_packets_total"), "Client transmit packets", CLIENT_LABELS)?,
            transmit_rate_bytes: gauge(registry, &format!("{ns}transmit_rate_bytes"), "Client transmit rate", CLIENT_LABELS)?,
            transmit_retries_total: gauge(registry, &format!("{ns}transmit_retries_total"), "Client transmit retries", CLIENT_LABELS)?,
            transfer_rate_bytes: gauge(registry, &format!("{ns}transfer_rate_bytes"), "Client data rate", CLIENT_LABELS)?,
            uptime_seconds: gauge(registry, &format!("{ns}uptime_seconds"), "Client uptime", CLIENT_LABELS)?,
            rssi_db: gauge(registry, &format!("{ns}rssi_db"), "Client RSSI", WIFI_LABELS)?,
            radio_signal_db: gauge(registry, &format!("{ns}radio_signal_db"), "Client signal", WIFI_LABELS)?,
            noise_db: gauge(registry, &format!("{ns}noise_db"), "Client AP noise", WIFI_LABELS)?,
            ccq_ratio: gauge(registry, &format!("{ns}ccq_ratio"), "Client connection quality", WIFI_LABELS)?,
            satisfaction_ratio: gauge(registry, &format!("{ns}satisfaction_ratio"), "Client satisfaction", WIFI_LABELS)?,
            roam_count_total: gauge(registry, &format!("{ns}roam_count_total"), "Client roam counter", WIFI_LABELS)?,
            radio_receive_mcs_index: gauge(registry, &format!("{ns}radio_receive_mcs_index"), "Receive MCS", WIFI_LABELS)?,
            radio_transmit_mcs_index: gauge(registry, &format!("{ns}radio_transmit_mcs_index"), "Transmit MCS", WIFI_LABELS)?,
            radio_receive_spatial_streams: gauge(registry, &format!("{ns}radio_receive_spatial_streams"), "Receive NSS", WIFI_LABELS)?,
            radio_transmit_spatial_streams: gauge(registry, &format!("{ns}radio_transmit_spatial_streams"), "Transmit NSS", WIFI_LABELS)?,
            radio_receive_rate_bps: gauge(registry, &format!("{ns}radio_receive_rate_bps"), "Receive PHY rate", WIFI_LABELS)?,
            radio_transmit_rate_bps: gauge(registry, &format!("{ns}radio_transmit_rate_bps"), "Transmit PHY rate", WIFI_LABELS)?,
            radio_transmit_power_dbm: gauge(registry, &format!("{ns}radio_transmit_power_dbm"), "Transmit power", WIFI_LABELS)?,
            wifi_attempts_transmit_total: gauge(registry, &format!("{ns}wifi_attempts_transmit_total"), "WiFi TX attempts", WIFI_LABELS)?,
            anomalies: gauge(registry, &format!("{ns}anomalies"), "Client anomalies", WIFI_LABELS)?,
            wired_receive_bytes_total: gauge(registry, "unifi_client_wired_receive_bytes_total", "Wired receive bytes", CLIENT_LABELS)?,
            wired_transmit_bytes_total: gauge(registry, "unifi_client_wired_transmit_bytes_total", "Wired transmit bytes", CLIENT_LABELS)?,
            wired_receive_packets_total: gauge(registry, "unifi_client_wired_receive_packets_total", "Wired receive packets", CLIENT_LABELS)?,
            wired_transmit_packets_total: gauge(registry, "unifi_client_wired_transmit_packets_total", "Wired transmit packets", CLIENT_LABELS)?,
            wired_receive_rate_bytes: gauge(registry, "unifi_client_wired_receive_rate_bytes", "Wired receive rate", CLIENT_LABELS)?,
            wired_transmit_rate_bytes: gauge(registry, "unifi_client_wired_transmit_rate_bytes", "Wired transmit rate", CLIENT_LABELS)?,
            dpi_transmit_packets: gauge(registry, "unifi_client_dpi_transmit_packets", "Client DPI transmit packets", DPI_LABELS)?,
            dpi_receive_packets: gauge(registry, "unifi_client_dpi_receive_packets", "Client DPI receive packets", DPI_LABELS)?,
            dpi_transmit_bytes: gauge(registry, "unifi_client_dpi_transmit_bytes", "Client DPI transmit bytes", DPI_LABELS)?,
            dpi_receive_bytes: gauge(registry, "unifi_client_dpi_receive_bytes", "Client DPI receive bytes", DPI_LABELS)?,
            count: IntGauge::with_opts(Opts::new("unifi_clients", "Connected client count"))?,
        };
        registry.register(Box::new(m.count.clone()))?;
        Ok(m)
    }

    pub fn apply(&self, snap: &Snapshot) {
        reset_all(self);
        let source = snap.source.as_str();
        let mut total = 0i64;

        for site in &snap.sites {
            let site_name = site.site.name.as_deref().unwrap_or("");
            let mac_names: std::collections::HashMap<String, String> = site
                .classic_devices
                .iter()
                .filter_map(|d| {
                    Some((d.mac.clone()?, d.display_name()))
                })
                .collect();

            if !site.classic_clients.is_empty() {
                for c in &site.classic_clients {
                    total += 1;
                    emit_classic(self, c, site_name, source, &mac_names);
                }
            } else {
                total += site.clients.len() as i64;
            }

            let client_names: std::collections::HashMap<String, String> = site
                .classic_clients
                .iter()
                .filter_map(|c| Some((c.mac.clone()?, c.display_name())))
                .collect();
            for dpi in &site.client_dpi {
                let mac = dpi.mac.as_deref().unwrap_or("");
                let name = dpi
                    .name
                    .clone()
                    .or_else(|| client_names.get(mac).cloned())
                    .unwrap_or_default();
                for e in &dpi.entries {
                    let labels = [
                        name.as_str(),
                        mac,
                        site_name,
                        source,
                        e.category(),
                        e.application(),
                    ];
                    set(&self.dpi_transmit_packets, &labels, e.tx_packets.get());
                    set(&self.dpi_receive_packets, &labels, e.rx_packets.get());
                    set(&self.dpi_transmit_bytes, &labels, e.tx_bytes.get());
                    set(&self.dpi_receive_bytes, &labels, e.rx_bytes.get());
                }
            }
        }
        self.count.set(total);
    }
}

fn emit_classic(
    m: &ClientMetrics,
    c: &ClassicClient,
    site_name: &str,
    source: &str,
    mac_names: &std::collections::HashMap<String, String>,
) {
    let name = c.display_name();
    let mac = c.mac.as_deref().unwrap_or("");
    let vlan = c.vlan.get().map(|v| format!("{v}")).unwrap_or_default();
    let sw_port = c.sw_port.get().map(|v| format!("{v}")).unwrap_or_default();
    let wired = if c.wired() { "true" } else { "false" };
    let gw = c
        .gw_mac
        .as_ref()
        .and_then(|mac| mac_names.get(mac).cloned())
        .unwrap_or_else(|| c.gw_mac.clone().unwrap_or_default());
    let sw = c
        .sw_mac
        .as_ref()
        .and_then(|mac| mac_names.get(mac).cloned())
        .unwrap_or_else(|| c.sw_mac.clone().unwrap_or_default());
    let ap = c
        .ap_mac
        .as_ref()
        .and_then(|mac| mac_names.get(mac).cloned())
        .unwrap_or_else(|| c.ap_mac.clone().unwrap_or_default());

    let base = [
        name.as_str(),
        mac,
        site_name,
        gw.as_str(),
        sw.as_str(),
        vlan.as_str(),
        c.ip.as_deref().unwrap_or(""),
        c.oui.as_deref().unwrap_or(""),
        c.network.as_deref().unwrap_or(""),
        sw_port.as_str(),
        ap.as_str(),
        source,
        "",
        wired,
    ];
    let channel = c.channel.get().map(|v| format!("{v}")).unwrap_or_default();
    let wifi = [
        c.radio_name.as_deref().unwrap_or(""),
        c.radio.as_deref().unwrap_or(""),
        c.radio_proto.as_deref().unwrap_or(""),
        channel.as_str(),
        c.essid.as_deref().unwrap_or(""),
        c.bssid.as_deref().unwrap_or(""),
        c.radio_desc.as_deref().unwrap_or(""),
        name.as_str(),
        mac,
        site_name,
        gw.as_str(),
        sw.as_str(),
        vlan.as_str(),
        c.ip.as_deref().unwrap_or(""),
        c.oui.as_deref().unwrap_or(""),
        c.network.as_deref().unwrap_or(""),
        sw_port.as_str(),
        ap.as_str(),
        source,
        "",
        wired,
    ];

    set(&m.receive_bytes_total, &base, c.rx_bytes.get());
    set(&m.receive_packets_total, &base, c.rx_packets.get());
    set(&m.receive_rate_bytes, &base, c.rx_bytes_r.get());
    set(&m.transmit_bytes_total, &base, c.tx_bytes.get());
    set(&m.transmit_packets_total, &base, c.tx_packets.get());
    set(&m.transmit_rate_bytes, &base, c.tx_bytes_r.get());
    set(&m.transmit_retries_total, &base, c.tx_retries.get());
    set(&m.transfer_rate_bytes, &base, c.bytes_r.get());
    set(&m.uptime_seconds, &base, c.uptime.get());
    set(&m.wired_receive_bytes_total, &base, c.wired_rx_bytes.get());
    set(&m.wired_transmit_bytes_total, &base, c.wired_tx_bytes.get());
    set(&m.wired_receive_packets_total, &base, c.wired_rx_packets.get());
    set(&m.wired_transmit_packets_total, &base, c.wired_tx_packets.get());
    set(&m.wired_receive_rate_bytes, &base, c.wired_rx_bytes_r.get());
    set(&m.wired_transmit_rate_bytes, &base, c.wired_tx_bytes_r.get());

    if !c.wired() {
        set(&m.rssi_db, &wifi, c.rssi.get());
        set(&m.radio_signal_db, &wifi, c.signal.get());
        set(&m.noise_db, &wifi, c.noise.get());
        set(
            &m.ccq_ratio,
            &wifi,
            c.ccq.get().map(|v| if v > 1.0 { v / 100.0 } else { v }),
        );
        set(
            &m.satisfaction_ratio,
            &wifi,
            c.satisfaction.get().map(|v| if v > 1.0 { v / 100.0 } else { v }),
        );
        set(&m.roam_count_total, &wifi, c.roam_count.get());
        set(&m.radio_receive_mcs_index, &wifi, c.rx_mcs.get());
        set(&m.radio_transmit_mcs_index, &wifi, c.tx_mcs.get());
        set(&m.radio_receive_spatial_streams, &wifi, c.rx_nss.get());
        set(&m.radio_transmit_spatial_streams, &wifi, c.tx_nss.get());
        set(
            &m.radio_receive_rate_bps,
            &wifi,
            c.rx_rate.get().map(|v| if v < 10_000.0 { v * 1000.0 } else { v }),
        );
        set(
            &m.radio_transmit_rate_bps,
            &wifi,
            c.tx_rate.get().map(|v| if v < 10_000.0 { v * 1000.0 } else { v }),
        );
        set(&m.radio_transmit_power_dbm, &wifi, c.tx_power.get());
        set(&m.wifi_attempts_transmit_total, &wifi, c.wifi_tx_attempts.get());
        set(&m.anomalies, &wifi, c.anomalies.get());
    }
}

fn gauge(registry: &Registry, name: &str, help: &str, labels: &[&str]) -> anyhow::Result<GaugeVec> {
    let g = GaugeVec::new(Opts::new(name, help), labels)?;
    registry.register(Box::new(g.clone()))?;
    Ok(g)
}

fn set(m: &GaugeVec, labels: &[&str], v: Option<f64>) {
    if let Some(v) = v {
        m.with_label_values(labels).set(v);
    }
}

fn reset_all(m: &ClientMetrics) {
    m.receive_bytes_total.reset();
    m.receive_packets_total.reset();
    m.receive_rate_bytes.reset();
    m.transmit_bytes_total.reset();
    m.transmit_packets_total.reset();
    m.transmit_rate_bytes.reset();
    m.transmit_retries_total.reset();
    m.transfer_rate_bytes.reset();
    m.uptime_seconds.reset();
    m.rssi_db.reset();
    m.radio_signal_db.reset();
    m.noise_db.reset();
    m.ccq_ratio.reset();
    m.satisfaction_ratio.reset();
    m.roam_count_total.reset();
    m.radio_receive_mcs_index.reset();
    m.radio_transmit_mcs_index.reset();
    m.radio_receive_spatial_streams.reset();
    m.radio_transmit_spatial_streams.reset();
    m.radio_receive_rate_bps.reset();
    m.radio_transmit_rate_bps.reset();
    m.radio_transmit_power_dbm.reset();
    m.wifi_attempts_transmit_total.reset();
    m.anomalies.reset();
    m.wired_receive_bytes_total.reset();
    m.wired_transmit_bytes_total.reset();
    m.wired_receive_packets_total.reset();
    m.wired_transmit_packets_total.reset();
    m.wired_receive_rate_bytes.reset();
    m.wired_transmit_rate_bytes.reset();
    m.dpi_transmit_packets.reset();
    m.dpi_receive_packets.reset();
    m.dpi_transmit_bytes.reset();
    m.dpi_receive_bytes.reset();
}
