use prometheus::{GaugeVec, IntGaugeVec, Registry};

use crate::poll::Snapshot;

use super::common::{gauge as g, igauge as ig, set, set_i};

pub struct ExtrasMetrics {
    site_health: IntGaugeVec,
    site_health_num_user: GaugeVec,
    site_health_num_guest: GaugeVec,
    site_health_latency: GaugeVec,
    site_health_rx_rate: GaugeVec,
    site_health_tx_rate: GaugeVec,
    site_stations: GaugeVec,
    site_uptime: GaugeVec,
    site_iots: GaugeVec,
    site_aps: GaugeVec,
    site_adopted: GaugeVec,
    site_disabled: GaugeVec,
    site_disconnected: GaugeVec,
    site_pending: GaugeVec,
    site_gateways: GaugeVec,
    site_switches: GaugeVec,
    site_drops: GaugeVec,
    site_xput_up: GaugeVec,
    site_xput_down: GaugeVec,
    site_speedtest_ping: GaugeVec,
    rogue_rssi: GaugeVec,
    rogue_age: GaugeVec,
    rogue_noise: GaugeVec,
    rogue_signal: GaugeVec,
    rogue_bw: GaugeVec,
    rogue_channel: GaugeVec,
    rogue_center_freq: GaugeVec,
    rogue_freq: GaugeVec,
    rogue_rssi_age: GaugeVec,
    dpi_tx_bytes: GaugeVec,
    dpi_rx_bytes: GaugeVec,
    dpi_tx_packets: GaugeVec,
    dpi_rx_packets: GaugeVec,
    speedtest_download: GaugeVec,
    speedtest_upload: GaugeVec,
    speedtest_latency: GaugeVec,
    speedtest_rundate: GaugeVec,
    speedtest_runtime: GaugeVec,
    controller_info: IntGaugeVec,
    controller_uptime: GaugeVec,
    controller_update_available: GaugeVec,
    controller_update_downloaded: GaugeVec,
    remote_user_active: GaugeVec,
    remote_user_inactive: GaugeVec,
    remote_user_rx_bytes: GaugeVec,
    remote_user_tx_bytes: GaugeVec,
    remote_user_rx_packets: GaugeVec,
    remote_user_tx_packets: GaugeVec,
    site_to_site_enabled: GaugeVec,
}

impl ExtrasMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        Ok(Self {
            site_health: ig(
                registry,
                "unifi_site_health_status",
                "1 if site subsystem status is ok",
                &["site_name", "subsystem", "status", "source"],
            )?,
            site_health_num_user: g(
                registry,
                "unifi_site_num_user",
                "Users reported by site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_health_num_guest: g(
                registry,
                "unifi_site_num_guest",
                "Guests reported by site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_health_latency: g(
                registry,
                "unifi_site_latency_seconds",
                "Site health latency",
                &["site_name", "subsystem", "source"],
            )?,
            site_health_rx_rate: g(
                registry,
                "unifi_site_receive_rate_bytes",
                "Site health RX rate",
                &["site_name", "subsystem", "source"],
            )?,
            site_health_tx_rate: g(
                registry,
                "unifi_site_transmit_rate_bytes",
                "Site health TX rate",
                &["site_name", "subsystem", "source"],
            )?,
            site_stations: g(
                registry,
                "unifi_site_stations",
                "Stations reported by site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_uptime: g(
                registry,
                "unifi_site_uptime_seconds",
                "Site health uptime",
                &["site_name", "subsystem", "source"],
            )?,
            site_iots: g(
                registry,
                "unifi_site_iots",
                "IoT devices reported by site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_aps: g(
                registry,
                "unifi_site_aps",
                "AP count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_adopted: g(
                registry,
                "unifi_site_adopted",
                "Adopted count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_disabled: g(
                registry,
                "unifi_site_disabled",
                "Disabled count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_disconnected: g(
                registry,
                "unifi_site_disconnected",
                "Disconnected count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_pending: g(
                registry,
                "unifi_site_pending",
                "Pending count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_gateways: g(
                registry,
                "unifi_site_gateways",
                "Gateway count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_switches: g(
                registry,
                "unifi_site_switches",
                "Switch count from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_drops: g(
                registry,
                "unifi_site_intenet_drops_total",
                "Internet (WAN) disconnections from site health",
                &["site_name", "subsystem", "source"],
            )?,
            site_xput_up: g(
                registry,
                "unifi_site_xput_up_rate",
                "Site health speedtest upload",
                &["site_name", "subsystem", "source"],
            )?,
            site_xput_down: g(
                registry,
                "unifi_site_xput_down_rate",
                "Site health speedtest download",
                &["site_name", "subsystem", "source"],
            )?,
            site_speedtest_ping: g(
                registry,
                "unifi_site_speedtest_ping",
                "Site health speedtest ping",
                &["site_name", "subsystem", "source"],
            )?,
            rogue_rssi: g(
                registry,
                "unifi_rogue_ap_rssi_db",
                "Rogue AP RSSI",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_age: g(
                registry,
                "unifi_rogue_ap_age_seconds",
                "Rogue AP age",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_noise: g(
                registry,
                "unifi_rogue_ap_noise_db",
                "Rogue AP noise",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_signal: g(
                registry,
                "unifi_rogue_ap_signal_db",
                "Rogue AP signal",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_bw: g(
                registry,
                "unifi_rogue_ap_bw",
                "Rogue AP bandwidth",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_channel: g(
                registry,
                "unifi_rogue_ap_channel",
                "Rogue AP channel number",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_center_freq: g(
                registry,
                "unifi_rogue_ap_center_freq",
                "Rogue AP center frequency",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_freq: g(
                registry,
                "unifi_rogue_ap_frequency",
                "Rogue AP frequency",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            rogue_rssi_age: g(
                registry,
                "unifi_rogue_ap_rssi_age_seconds",
                "Rogue AP RSSI age",
                &["site_name", "essid", "bssid", "channel", "source"],
            )?,
            dpi_tx_bytes: g(
                registry,
                "unifi_dpi_transmit_bytes",
                "DPI transmit bytes",
                &["site_name", "category", "application", "source"],
            )?,
            dpi_rx_bytes: g(
                registry,
                "unifi_dpi_receive_bytes",
                "DPI receive bytes",
                &["site_name", "category", "application", "source"],
            )?,
            dpi_tx_packets: g(
                registry,
                "unifi_dpi_transmit_packets",
                "DPI transmit packets",
                &["site_name", "category", "application", "source"],
            )?,
            dpi_rx_packets: g(
                registry,
                "unifi_dpi_receive_packets",
                "DPI receive packets",
                &["site_name", "category", "application", "source"],
            )?,
            speedtest_download: g(
                registry,
                "unifi_device_speedtest_download",
                "Last speedtest download (Mbps)",
                &["site_name", "source"],
            )?,
            speedtest_upload: g(
                registry,
                "unifi_device_speedtest_upload",
                "Last speedtest upload (Mbps)",
                &["site_name", "source"],
            )?,
            speedtest_latency: g(
                registry,
                "unifi_device_speedtest_latency_seconds",
                "Last speedtest latency",
                &["site_name", "source"],
            )?,
            speedtest_rundate: g(
                registry,
                "unifi_device_speedtest_rundate_seconds",
                "Last speedtest unix time",
                &["site_name", "source"],
            )?,
            speedtest_runtime: g(
                registry,
                "unifi_device_speedtest_runtime_seconds",
                "Last speedtest runtime",
                &["site_name", "source"],
            )?,
            controller_info: ig(
                registry,
                "unifi_controller_info",
                "Controller identity from /stat/sysinfo (value 1)",
                &[
                    "site_name",
                    "hostname",
                    "name",
                    "version",
                    "device_type",
                    "timezone",
                    "source",
                ],
            )?,
            controller_uptime: g(
                registry,
                "unifi_controller_uptime_seconds",
                "Controller uptime from /stat/sysinfo",
                &["site_name", "hostname", "source"],
            )?,
            controller_update_available: g(
                registry,
                "unifi_controller_update_available",
                "1 if a controller update is available",
                &["site_name", "hostname", "version", "source"],
            )?,
            controller_update_downloaded: g(
                registry,
                "unifi_controller_update_downloaded",
                "1 if a controller update is downloaded",
                &["site_name", "hostname", "source"],
            )?,
            remote_user_active: g(
                registry,
                "unifi_site_remote_user_active",
                "Remote VPN users active",
                &["site_name", "subsystem", "source"],
            )?,
            remote_user_inactive: g(
                registry,
                "unifi_site_remote_user_inactive",
                "Remote VPN users inactive",
                &["site_name", "subsystem", "source"],
            )?,
            remote_user_rx_bytes: g(
                registry,
                "unifi_site_remote_user_receive_bytes_total",
                "Remote VPN user RX bytes",
                &["site_name", "subsystem", "source"],
            )?,
            remote_user_tx_bytes: g(
                registry,
                "unifi_site_remote_user_transmit_bytes_total",
                "Remote VPN user TX bytes",
                &["site_name", "subsystem", "source"],
            )?,
            remote_user_rx_packets: g(
                registry,
                "unifi_site_remote_user_receive_packets_total",
                "Remote VPN user RX packets",
                &["site_name", "subsystem", "source"],
            )?,
            remote_user_tx_packets: g(
                registry,
                "unifi_site_remote_user_transmit_packets_total",
                "Remote VPN user TX packets",
                &["site_name", "subsystem", "source"],
            )?,
            site_to_site_enabled: g(
                registry,
                "unifi_site_site_to_site_enabled",
                "Site-to-site VPN enabled from health",
                &["site_name", "subsystem", "source"],
            )?,
        })
    }

    pub fn apply(&self, snap: &Snapshot) {
        self.site_health.reset();
        self.site_health_num_user.reset();
        self.site_health_num_guest.reset();
        self.site_health_latency.reset();
        self.site_health_rx_rate.reset();
        self.site_health_tx_rate.reset();
        self.site_stations.reset();
        self.site_uptime.reset();
        self.site_iots.reset();
        self.site_aps.reset();
        self.site_adopted.reset();
        self.site_disabled.reset();
        self.site_disconnected.reset();
        self.site_pending.reset();
        self.site_gateways.reset();
        self.site_switches.reset();
        self.site_drops.reset();
        self.site_xput_up.reset();
        self.site_xput_down.reset();
        self.site_speedtest_ping.reset();
        self.rogue_rssi.reset();
        self.rogue_age.reset();
        self.rogue_noise.reset();
        self.rogue_signal.reset();
        self.rogue_bw.reset();
        self.rogue_channel.reset();
        self.rogue_center_freq.reset();
        self.rogue_freq.reset();
        self.rogue_rssi_age.reset();
        self.dpi_tx_bytes.reset();
        self.dpi_rx_bytes.reset();
        self.dpi_tx_packets.reset();
        self.dpi_rx_packets.reset();
        self.speedtest_download.reset();
        self.speedtest_upload.reset();
        self.speedtest_latency.reset();
        self.speedtest_rundate.reset();
        self.speedtest_runtime.reset();
        self.controller_info.reset();
        self.controller_uptime.reset();
        self.controller_update_available.reset();
        self.controller_update_downloaded.reset();
        self.remote_user_active.reset();
        self.remote_user_inactive.reset();
        self.remote_user_rx_bytes.reset();
        self.remote_user_tx_bytes.reset();
        self.remote_user_rx_packets.reset();
        self.remote_user_tx_packets.reset();
        self.site_to_site_enabled.reset();

        let source = snap.source.as_str();
        for site in &snap.sites {
            let site_name = site.site.name.as_deref().unwrap_or("");
            for h in &site.health {
                let sub = h.subsystem.as_deref().unwrap_or("");
                let status = h.status.as_deref().unwrap_or("");
                let ok = if status.eq_ignore_ascii_case("ok") {
                    1
                } else {
                    0
                };
                self.site_health
                    .with_label_values(&[site_name, sub, status, source])
                    .set(ok);
                let hl = [site_name, sub, source];
                set(&self.site_health_num_user, &hl, h.num_user.get());
                set(&self.site_health_num_guest, &hl, h.num_guest.get());
                set(
                    &self.site_health_latency,
                    &hl,
                    h.latency.get().map(|v| if v > 10.0 { v / 1000.0 } else { v }),
                );
                set(&self.site_health_rx_rate, &hl, h.rx_bytes_r.get());
                set(&self.site_health_tx_rate, &hl, h.tx_bytes_r.get());
                set(&self.site_stations, &hl, h.num_sta.get());
                set(&self.site_uptime, &hl, h.uptime.get());
                set(&self.site_iots, &hl, h.num_iot.get());
                set(&self.site_aps, &hl, h.num_ap.get());
                set(&self.site_adopted, &hl, h.num_adopted.get());
                set(&self.site_disabled, &hl, h.num_disabled.get());
                set(&self.site_disconnected, &hl, h.num_disconnected.get());
                set(&self.site_pending, &hl, h.num_pending.get());
                set(&self.site_gateways, &hl, h.num_gw.get());
                set(&self.site_switches, &hl, h.num_sw.get());
                set(&self.site_drops, &hl, h.drops.get());
                set(&self.site_xput_up, &hl, h.xput_up.get());
                set(&self.site_xput_down, &hl, h.xput_down.get());
                set(&self.site_speedtest_ping, &hl, h.speedtest_ping.get());
                set(&self.remote_user_active, &hl, h.remote_user_num_active.get());
                set(&self.remote_user_inactive, &hl, h.remote_user_num_inactive.get());
                set(&self.remote_user_rx_bytes, &hl, h.remote_user_rx_bytes.get());
                set(&self.remote_user_tx_bytes, &hl, h.remote_user_tx_bytes.get());
                set(&self.remote_user_rx_packets, &hl, h.remote_user_rx_packets.get());
                set(&self.remote_user_tx_packets, &hl, h.remote_user_tx_packets.get());
                set(
                    &self.site_to_site_enabled,
                    &hl,
                    h.site_to_site_enabled.get().map(|v| if v != 0.0 { 1.0 } else { 0.0 }),
                );
            }
            for r in &site.rogue_aps {
                let ch = r.channel.label();
                let rl = [
                    site_name,
                    r.essid.as_deref().unwrap_or(""),
                    r.bssid.as_deref().unwrap_or(""),
                    ch.as_str(),
                    source,
                ];
                set(&self.rogue_rssi, &rl, r.rssi.get());
                set(&self.rogue_age, &rl, r.age.get());
                set(&self.rogue_noise, &rl, r.noise.get());
                set(&self.rogue_signal, &rl, r.signal.get());
                set(&self.rogue_bw, &rl, r.bw.get());
                set(&self.rogue_channel, &rl, r.channel.get());
                set(&self.rogue_center_freq, &rl, r.center_freq.get());
                set(&self.rogue_freq, &rl, r.freq.get());
                set(&self.rogue_rssi_age, &rl, r.rssi_age.get());
            }
            for d in &site.dpi {
                let dl = [site_name, d.category(), d.application(), source];
                set(&self.dpi_tx_bytes, &dl, d.tx_bytes.get());
                set(&self.dpi_rx_bytes, &dl, d.rx_bytes.get());
                set(&self.dpi_tx_packets, &dl, d.tx_packets.get());
                set(&self.dpi_rx_packets, &dl, d.rx_packets.get());
            }
            if let Some(st) = &site.speedtest {
                set(&self.speedtest_download, &[site_name, source], st.xput_download.get());
                set(&self.speedtest_upload, &[site_name, source], st.xput_upload.get());
                set(
                    &self.speedtest_latency,
                    &[site_name, source],
                    st.latency.get().map(|v| if v > 10.0 { v / 1000.0 } else { v }),
                );
                set(&self.speedtest_rundate, &[site_name, source], st.rundate.get());
                set(&self.speedtest_runtime, &[site_name, source], st.runtime.get());
            }
            if let Some(info) = &site.sysinfo {
                let hostname = info.hostname.as_deref().unwrap_or("");
                let name = info.name.as_deref().unwrap_or("");
                let version = info.version.as_deref().unwrap_or("");
                set_i(
                    &self.controller_info,
                    &[
                        site_name,
                        hostname,
                        name,
                        version,
                        info.ubnt_device_type.as_deref().unwrap_or(""),
                        info.timezone.as_deref().unwrap_or(""),
                        source,
                    ],
                    1,
                );
                set(
                    &self.controller_uptime,
                    &[site_name, hostname, source],
                    info.uptime.get(),
                );
                set(
                    &self.controller_update_available,
                    &[site_name, hostname, version, source],
                    info.update_available.get().map(|v| if v != 0.0 { 1.0 } else { 0.0 }),
                );
                set(
                    &self.controller_update_downloaded,
                    &[site_name, hostname, source],
                    info.update_downloaded.get().map(|v| if v != 0.0 { 1.0 } else { 0.0 }),
                );
            }
        }
    }
}
