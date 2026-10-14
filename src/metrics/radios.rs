use prometheus::{GaugeVec, Registry};

use crate::client::classic::models::{radio_band, ClassicDevice, Radio, Vap};
use crate::poll::{DeviceWithStats, Snapshot};

use super::common::{ccq_ratio, gauge, ms_to_sec, ratio, set};

const RADIO: &[&str] = &["radio_name", "radio", "band", "mac", "site_name", "name", "source"];
const RADIO_STA: &[&str] = &[
    "radio_name", "radio", "band", "mac", "site_name", "name", "source", "station_type",
];
const VAP: &[&str] = &[
    "vap_name",
    "bssid",
    "radio",
    "band",
    "radio_name",
    "essid",
    "usage",
    "mac",
    "site_name",
    "name",
    "source",
];

pub struct RadioMetrics {
    current_antenna_gain: GaugeVec,
    ht: GaugeVec,
    max_transmit_power: GaugeVec,
    min_transmit_power: GaugeVec,
    nss: GaugeVec,
    caps: GaugeVec,
    transmit_power: GaugeVec,
    ast_be_xmit: GaugeVec,
    channel: GaugeVec,
    cu_rx: GaugeVec,
    cu_tx: GaugeVec,
    cu_total: GaugeVec,
    ext_channel: GaugeVec,
    gain: GaugeVec,
    stations: GaugeVec,
    transmit_packets: GaugeVec,
    transmit_retries: GaugeVec,
    transmit_retries_ratio: GaugeVec,
    vap_ccq_ratio: GaugeVec,
    vap_mac_filter_rejects_total: GaugeVec,
    vap_satisfaction_stations: GaugeVec,
    vap_average_client_signal: GaugeVec,
    vap_satisfaction_ratio: GaugeVec,
    vap_satisfaction_now_ratio: GaugeVec,
    vap_receive_bytes_total: GaugeVec,
    vap_receive_crypts_total: GaugeVec,
    vap_receive_dropped_total: GaugeVec,
    vap_receive_errors_total: GaugeVec,
    vap_receive_frags_total: GaugeVec,
    vap_receive_nwids_total: GaugeVec,
    vap_receive_packets_total: GaugeVec,
    vap_transmit_bytes_total: GaugeVec,
    vap_transmit_dropped_total: GaugeVec,
    vap_transmit_errors_total: GaugeVec,
    vap_transmit_packets_total: GaugeVec,
    vap_transmit_power: GaugeVec,
    vap_transmit_retries_total: GaugeVec,
    vap_transmit_retries_combined_total: GaugeVec,
    vap_transmit_rts_retries_total: GaugeVec,
    vap_transmit_total: GaugeVec,
    vap_dns_latency: GaugeVec,
    vap_tx_mpdu: GaugeVec,
    vap_tx_goodbytes: GaugeVec,
    vap_tx_lat_avg: GaugeVec,
    vap_tx_lat_max: GaugeVec,
    vap_tx_lat_min: GaugeVec,
    vap_rx_goodbytes: GaugeVec,
    vap_rx_lat_avg: GaugeVec,
    vap_rx_lat_max: GaugeVec,
    vap_rx_lat_min: GaugeVec,
    vap_tx_mov_avg: GaugeVec,
    vap_tx_mov_max: GaugeVec,
    vap_tx_mov_min: GaugeVec,
    vap_tx_mov_total: GaugeVec,
    vap_tx_mov_count: GaugeVec,
}

impl RadioMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let ns = "unifi_device_";
        Ok(Self {
            current_antenna_gain: gauge(registry, &format!("{ns}radio_current_antenna_gain"), "Radio antenna gain", RADIO)?,
            ht: gauge(registry, &format!("{ns}radio_ht"), "Radio HT", RADIO)?,
            max_transmit_power: gauge(registry, &format!("{ns}radio_max_transmit_power"), "Radio max TX power", RADIO)?,
            min_transmit_power: gauge(registry, &format!("{ns}radio_min_transmit_power"), "Radio min TX power", RADIO)?,
            nss: gauge(registry, &format!("{ns}radio_nss"), "Radio NSS", RADIO)?,
            caps: gauge(registry, &format!("{ns}radio_caps"), "Radio capabilities", RADIO)?,
            transmit_power: gauge(registry, &format!("{ns}radio_transmit_power"), "Radio TX power", RADIO)?,
            ast_be_xmit: gauge(registry, &format!("{ns}radio_ast_be_xmit"), "Radio AstBe transmit", RADIO)?,
            channel: gauge(registry, &format!("{ns}radio_channel"), "Radio channel", RADIO)?,
            cu_rx: gauge(registry, &format!("{ns}radio_channel_utilization_receive_ratio"), "Channel utilization RX", RADIO)?,
            cu_tx: gauge(registry, &format!("{ns}radio_channel_utilization_transmit_ratio"), "Channel utilization TX", RADIO)?,
            cu_total: gauge(registry, &format!("{ns}radio_channel_utilization_total_ratio"), "Channel utilization total", RADIO)?,
            ext_channel: gauge(registry, &format!("{ns}radio_ext_channel"), "Radio ext channel", RADIO)?,
            gain: gauge(registry, &format!("{ns}radio_gain"), "Radio gain", RADIO)?,
            stations: gauge(registry, &format!("{ns}radio_stations"), "Radio station count", RADIO_STA)?,
            transmit_packets: gauge(registry, &format!("{ns}radio_transmit_packets"), "Radio TX packets", RADIO)?,
            transmit_retries: gauge(registry, &format!("{ns}radio_transmit_retries"), "Radio TX retries", RADIO)?,
            transmit_retries_ratio: gauge(
                registry,
                &format!("{ns}radio_transmit_retries_ratio"),
                "Radio TX retries as a ratio (Integration statistics/latest)",
                RADIO,
            )?,
            vap_ccq_ratio: gauge(registry, &format!("{ns}vap_ccq_ratio"), "VAP CCQ", VAP)?,
            vap_mac_filter_rejects_total: gauge(registry, &format!("{ns}vap_mac_filter_rejects_total"), "VAP MAC filter rejects", VAP)?,
            vap_satisfaction_stations: gauge(registry, &format!("{ns}vap_satisfaction_stations"), "VAP station count", VAP)?,
            vap_average_client_signal: gauge(registry, &format!("{ns}vap_average_client_signal"), "VAP average client signal", VAP)?,
            vap_satisfaction_ratio: gauge(registry, &format!("{ns}vap_satisfaction_ratio"), "VAP satisfaction", VAP)?,
            vap_satisfaction_now_ratio: gauge(registry, &format!("{ns}vap_satisfaction_now_ratio"), "VAP satisfaction now", VAP)?,
            vap_receive_bytes_total: gauge(registry, &format!("{ns}vap_receive_bytes_total"), "VAP RX bytes", VAP)?,
            vap_receive_crypts_total: gauge(registry, &format!("{ns}vap_receive_crypts_total"), "VAP RX crypts", VAP)?,
            vap_receive_dropped_total: gauge(registry, &format!("{ns}vap_receive_dropped_total"), "VAP RX dropped", VAP)?,
            vap_receive_errors_total: gauge(registry, &format!("{ns}vap_receive_errors_total"), "VAP RX errors", VAP)?,
            vap_receive_frags_total: gauge(registry, &format!("{ns}vap_receive_frags_total"), "VAP RX frags", VAP)?,
            vap_receive_nwids_total: gauge(registry, &format!("{ns}vap_receive_nwids_total"), "VAP RX nwids", VAP)?,
            vap_receive_packets_total: gauge(registry, &format!("{ns}vap_receive_packets_total"), "VAP RX packets", VAP)?,
            vap_transmit_bytes_total: gauge(registry, &format!("{ns}vap_transmit_bytes_total"), "VAP TX bytes", VAP)?,
            vap_transmit_dropped_total: gauge(registry, &format!("{ns}vap_transmit_dropped_total"), "VAP TX dropped", VAP)?,
            vap_transmit_errors_total: gauge(registry, &format!("{ns}vap_transmit_errors_total"), "VAP TX errors", VAP)?,
            vap_transmit_packets_total: gauge(registry, &format!("{ns}vap_transmit_packets_total"), "VAP TX packets", VAP)?,
            vap_transmit_power: gauge(registry, &format!("{ns}vap_transmit_power"), "VAP TX power", VAP)?,
            vap_transmit_retries_total: gauge(registry, &format!("{ns}vap_transmit_retries_total"), "VAP TX retries", VAP)?,
            vap_transmit_retries_combined_total: gauge(registry, &format!("{ns}vap_transmit_retries_combined_total"), "VAP combined TX retries", VAP)?,
            vap_transmit_rts_retries_total: gauge(registry, &format!("{ns}vap_transmit_rts_retries_total"), "VAP RTS retries", VAP)?,
            vap_transmit_total: gauge(registry, &format!("{ns}vap_transmit_total"), "VAP TX total", VAP)?,
            vap_dns_latency: gauge(registry, &format!("{ns}vap_dns_latency_average_seconds"), "VAP DNS latency", VAP)?,
            vap_tx_mpdu: gauge(registry, &format!("{ns}vap_data_mpdu_transmit_bytes_total"), "VAP data MPDU TX bytes", VAP)?,
            vap_tx_goodbytes: gauge(registry, &format!("{ns}vap_transmit_goodbyes"), "VAP TCP goodbytes TX", VAP)?,
            vap_tx_lat_avg: gauge(registry, &format!("{ns}vap_transmit_latency_average_seconds"), "VAP TCP latency avg TX", VAP)?,
            vap_tx_lat_max: gauge(registry, &format!("{ns}vap_transmit_latency_maximum_seconds"), "VAP TCP latency max TX", VAP)?,
            vap_tx_lat_min: gauge(registry, &format!("{ns}vap_transmit_latency_minimum_seconds"), "VAP TCP latency min TX", VAP)?,
            vap_rx_goodbytes: gauge(registry, &format!("{ns}vap_receive_goodbyes"), "VAP TCP goodbytes RX", VAP)?,
            vap_rx_lat_avg: gauge(registry, &format!("{ns}vap_receive_latency_average_seconds"), "VAP TCP latency avg RX", VAP)?,
            vap_rx_lat_max: gauge(registry, &format!("{ns}vap_receive_latency_maximum_seconds"), "VAP TCP latency max RX", VAP)?,
            vap_rx_lat_min: gauge(registry, &format!("{ns}vap_receive_latency_minimum_seconds"), "VAP TCP latency min RX", VAP)?,
            vap_tx_mov_avg: gauge(registry, &format!("{ns}vap_transmit_latency_moving_avg_seconds"), "VAP TX latency moving avg", VAP)?,
            vap_tx_mov_max: gauge(registry, &format!("{ns}vap_transmit_latency_moving_max_seconds"), "VAP TX latency moving max", VAP)?,
            vap_tx_mov_min: gauge(registry, &format!("{ns}vap_transmit_latency_moving_min_seconds"), "VAP TX latency moving min", VAP)?,
            vap_tx_mov_total: gauge(registry, &format!("{ns}vap_transmit_latency_moving_total"), "VAP TX latency moving total", VAP)?,
            vap_tx_mov_count: gauge(registry, &format!("{ns}vap_transmit_latency_moving_count"), "VAP TX latency moving count", VAP)?,
        })
    }

    pub fn apply(&self, snap: &Snapshot) {
        self.reset();
        let source = snap.source.as_str();
        for site in &snap.sites {
            let site_name = site.site.name.as_deref().unwrap_or("");
            if !site.classic_devices.is_empty() {
                for d in &site.classic_devices {
                    emit_classic(self, d, site_name, source);
                }
            } else {
                for dws in &site.devices {
                    emit_integration(self, dws, site_name, source);
                }
            }
        }
    }

    fn reset(&self) {
        self.current_antenna_gain.reset();
        self.ht.reset();
        self.max_transmit_power.reset();
        self.min_transmit_power.reset();
        self.nss.reset();
        self.caps.reset();
        self.transmit_power.reset();
        self.ast_be_xmit.reset();
        self.channel.reset();
        self.cu_rx.reset();
        self.cu_tx.reset();
        self.cu_total.reset();
        self.ext_channel.reset();
        self.gain.reset();
        self.stations.reset();
        self.transmit_packets.reset();
        self.transmit_retries.reset();
        self.transmit_retries_ratio.reset();
        self.vap_ccq_ratio.reset();
        self.vap_mac_filter_rejects_total.reset();
        self.vap_satisfaction_stations.reset();
        self.vap_average_client_signal.reset();
        self.vap_satisfaction_ratio.reset();
        self.vap_satisfaction_now_ratio.reset();
        self.vap_receive_bytes_total.reset();
        self.vap_receive_crypts_total.reset();
        self.vap_receive_dropped_total.reset();
        self.vap_receive_errors_total.reset();
        self.vap_receive_frags_total.reset();
        self.vap_receive_nwids_total.reset();
        self.vap_receive_packets_total.reset();
        self.vap_transmit_bytes_total.reset();
        self.vap_transmit_dropped_total.reset();
        self.vap_transmit_errors_total.reset();
        self.vap_transmit_packets_total.reset();
        self.vap_transmit_power.reset();
        self.vap_transmit_retries_total.reset();
        self.vap_transmit_retries_combined_total.reset();
        self.vap_transmit_rts_retries_total.reset();
        self.vap_transmit_total.reset();
        self.vap_dns_latency.reset();
        self.vap_tx_mpdu.reset();
        self.vap_tx_goodbytes.reset();
        self.vap_tx_lat_avg.reset();
        self.vap_tx_lat_max.reset();
        self.vap_tx_lat_min.reset();
        self.vap_rx_goodbytes.reset();
        self.vap_rx_lat_avg.reset();
        self.vap_rx_lat_max.reset();
        self.vap_rx_lat_min.reset();
        self.vap_tx_mov_avg.reset();
        self.vap_tx_mov_max.reset();
        self.vap_tx_mov_min.reset();
        self.vap_tx_mov_total.reset();
        self.vap_tx_mov_count.reset();
    }
}

fn emit_classic(m: &RadioMetrics, d: &ClassicDevice, site_name: &str, source: &str) {
    let name = d.display_name();
    let mac = d.mac.as_deref().unwrap_or("");
    let radios = if d.radio_table.is_empty() {
        d.radio_table_stats.as_slice()
    } else {
        d.radio_table.as_slice()
    };
    for r in radios {
        let stats = d
            .radio_table_stats
            .iter()
            .find(|s| s.name == r.name)
            .unwrap_or(r);
        emit_radio(m, r, stats, site_name, name.as_str(), mac, source);
    }
    for v in &d.vap_table {
        if v.is_up() {
            emit_vap(m, v, site_name, name.as_str(), mac, source);
        }
    }
}

fn emit_radio(
    m: &RadioMetrics,
    cfg: &Radio,
    stats: &Radio,
    site_name: &str,
    name: &str,
    mac: &str,
    source: &str,
) {
    let radio = cfg.radio.as_deref().or(stats.radio.as_deref()).unwrap_or("");
    let band = radio_band(radio);
    let rname = cfg.name.as_deref().or(stats.name.as_deref()).unwrap_or("");
    let labels = [rname, radio, band, mac, site_name, name, source];
    set(&m.current_antenna_gain, &labels, cfg.current_antenna_gain.get());
    set(&m.ht, &labels, cfg.ht.get());
    set(&m.max_transmit_power, &labels, cfg.max_txpower.get());
    set(&m.min_transmit_power, &labels, cfg.min_txpower.get());
    set(&m.nss, &labels, cfg.nss.get());
    set(&m.caps, &labels, cfg.radio_caps.get());
    set(&m.transmit_power, &labels, stats.tx_power.get().or_else(|| cfg.tx_power.get()));
    set(&m.ast_be_xmit, &labels, stats.ast_be_xmit.get());
    set(&m.channel, &labels, stats.channel.get().or_else(|| cfg.channel.get()));
    set(&m.cu_rx, &labels, ratio(stats.cu_self_rx.get().or_else(|| cfg.cu_self_rx.get())));
    set(&m.cu_tx, &labels, ratio(stats.cu_self_tx.get().or_else(|| cfg.cu_self_tx.get())));
    set(&m.cu_total, &labels, ratio(stats.cu_total.get().or_else(|| cfg.cu_total.get())));
    set(&m.ext_channel, &labels, stats.extchannel.get());
    set(&m.gain, &labels, stats.gain.get().or_else(|| cfg.gain.get()));
    set(
        &m.stations,
        &[rname, radio, band, mac, site_name, name, source, "user"],
        stats.user_num_sta.get().or_else(|| cfg.user_num_sta.get()),
    );
    set(
        &m.stations,
        &[rname, radio, band, mac, site_name, name, source, "guest"],
        stats.guest_num_sta.get().or_else(|| cfg.guest_num_sta.get()),
    );
    set(&m.transmit_packets, &labels, stats.tx_packets.get());
    set(&m.transmit_retries, &labels, stats.tx_retries.get().or_else(|| cfg.tx_retries.get()));
}

fn emit_vap(m: &RadioMetrics, v: &Vap, site_name: &str, name: &str, mac: &str, source: &str) {
    let labels = [
        v.name.as_deref().unwrap_or(""),
        v.bssid.as_deref().unwrap_or(""),
        v.radio.as_deref().unwrap_or(""),
        v.band(),
        v.radio_name.as_deref().unwrap_or(""),
        v.essid.as_deref().unwrap_or(""),
        v.usage.as_deref().unwrap_or(""),
        mac,
        site_name,
        name,
        source,
    ];
    set(&m.vap_ccq_ratio, &labels, ccq_ratio(v.ccq.get()));
    set(&m.vap_mac_filter_rejects_total, &labels, v.mac_filter_rejections.get());
    set(
        &m.vap_satisfaction_stations,
        &labels,
        v.num_satisfaction_sta.get().or_else(|| v.num_sta.get()),
    );
    set(&m.vap_average_client_signal, &labels, v.avg_client_signal.get());
    set(&m.vap_satisfaction_ratio, &labels, ratio(v.satisfaction.get()));
    set(&m.vap_satisfaction_now_ratio, &labels, ratio(v.satisfaction_now.get()));
    set(&m.vap_receive_bytes_total, &labels, v.rx_bytes.get());
    set(&m.vap_receive_crypts_total, &labels, v.rx_crypts.get());
    set(&m.vap_receive_dropped_total, &labels, v.rx_dropped.get());
    set(&m.vap_receive_errors_total, &labels, v.rx_errors.get());
    set(&m.vap_receive_frags_total, &labels, v.rx_frags.get());
    set(&m.vap_receive_nwids_total, &labels, v.rx_nwids.get());
    set(&m.vap_receive_packets_total, &labels, v.rx_packets.get());
    set(&m.vap_transmit_bytes_total, &labels, v.tx_bytes.get());
    set(&m.vap_transmit_dropped_total, &labels, v.tx_dropped.get());
    set(&m.vap_transmit_errors_total, &labels, v.tx_errors.get());
    set(&m.vap_transmit_packets_total, &labels, v.tx_packets.get());
    set(&m.vap_transmit_power, &labels, v.tx_power.get());
    set(&m.vap_transmit_retries_total, &labels, v.tx_retries.get());
    set(&m.vap_transmit_retries_combined_total, &labels, v.tx_combined_retries.get());
    set(&m.vap_transmit_rts_retries_total, &labels, v.tx_rts_retries.get());
    set(&m.vap_transmit_total, &labels, v.tx_total.get());
    set(&m.vap_dns_latency, &labels, ms_to_sec(v.dns_avg_latency.get()));
    set(&m.vap_tx_mpdu, &labels, v.tx_data_mpdu_bytes.get());
    if let Some(tx) = &v.tx_tcp_stats {
        set(&m.vap_tx_goodbytes, &labels, tx.goodbytes.get());
        set(&m.vap_tx_lat_avg, &labels, ms_to_sec(tx.lat_avg.get()));
        set(&m.vap_tx_lat_max, &labels, ms_to_sec(tx.lat_max.get()));
        set(&m.vap_tx_lat_min, &labels, ms_to_sec(tx.lat_min.get()));
    }
    if let Some(rx) = &v.rx_tcp_stats {
        set(&m.vap_rx_goodbytes, &labels, rx.goodbytes.get());
        set(&m.vap_rx_lat_avg, &labels, ms_to_sec(rx.lat_avg.get()));
        set(&m.vap_rx_lat_max, &labels, ms_to_sec(rx.lat_max.get()));
        set(&m.vap_rx_lat_min, &labels, ms_to_sec(rx.lat_min.get()));
    }
    if let Some(mov) = &v.wifi_tx_latency_mov {
        set(&m.vap_tx_mov_avg, &labels, ms_to_sec(mov.avg.get()));
        set(&m.vap_tx_mov_max, &labels, ms_to_sec(mov.max.get()));
        set(&m.vap_tx_mov_min, &labels, ms_to_sec(mov.min.get()));
        set(&m.vap_tx_mov_total, &labels, mov.total.get());
        set(&m.vap_tx_mov_count, &labels, mov.total_count.get());
    }
}

fn emit_integration(m: &RadioMetrics, dws: &DeviceWithStats, site_name: &str, source: &str) {
    let d = &dws.device;
    let name = d.name.clone().unwrap_or_default();
    let mac = d.mac_address.as_deref().unwrap_or("");
    for (i, r) in d.interfaces.radios.iter().enumerate() {
        let band = r
            .frequency_g_hz
            .map(|f| {
                if (f - 2.4).abs() < 0.2 {
                    "2.4"
                } else if (f - 5.0).abs() < 1.0 {
                    "5"
                } else if (f - 6.0).abs() < 1.0 {
                    "6"
                } else {
                    ""
                }
            })
            .unwrap_or("");
        let rname = format!("radio{i}");
        let labels = [rname.as_str(), "", band, mac, site_name, name.as_str(), source];
        set(&m.channel, &labels, r.channel.map(|v| v as f64));
        if let Some(stats) = &dws.stats {
            if let Some(ifaces) = &stats.interfaces {
                if let Some(rs) = ifaces.radios.get(i) {
                    set(
                        &m.transmit_retries_ratio,
                        &labels,
                        rs.tx_retries_pct.map(|v| if v > 1.0 { v / 100.0 } else { v }),
                    );
                }
            }
        }
    }
}
