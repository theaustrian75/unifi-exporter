use prometheus::{GaugeVec, IntGauge, IntGaugeVec, Opts, Registry};

use crate::client::classic::models::ClassicDevice;
use crate::poll::Snapshot;

use super::common::{gauge, igauge, set};

const LABELS: &[&str] = &["type", "site_name", "name", "source"];
const INFO_LABELS: &[&str] = &[
    "type", "site_name", "name", "source", "version", "model", "serial", "mac", "ip", "id",
];
const TEMP_LABELS: &[&str] = &["type", "site_name", "name", "source", "temp_area", "temp_type"];
const STA_LABELS: &[&str] = &["type", "site_name", "name", "source", "station_type"];
const STAT_LABELS: &[&str] = &["stat", "site_name", "name", "source"];
const STORAGE_LABELS: &[&str] = &[
    "type", "site_name", "name", "source", "mountpoint", "storage_name", "storage_reading",
];
const OUTLET: &[&str] = &[
    "outlet_description", "outlet_index", "outlet_name", "site_name", "name", "source",
];
const UPS: &[&str] = &["site_name", "device_name", "device_mac", "source"];
const CI_INFO: &[&str] = &[
    "type", "site_name", "name", "source", "ci_state", "ci_sw_dl_status", "ci_mac", "ci_version",
    "ci_mode",
];

pub struct DeviceMetrics {
    info: IntGaugeVec,
    present: IntGaugeVec,
    uptime_seconds: GaugeVec,
    cpu_utilization_ratio: GaugeVec,
    memory_utilization_ratio: GaugeVec,
    load_average_1: GaugeVec,
    load_average_5: GaugeVec,
    load_average_15: GaugeVec,
    memory_used_bytes: GaugeVec,
    memory_installed_bytes: GaugeVec,
    memory_buffer_bytes: GaugeVec,
    transmit_bytes_total: GaugeVec,
    receive_bytes_total: GaugeVec,
    bytes_total: GaugeVec,
    temperature_celsius: GaugeVec,
    stations: GaugeVec,
    upgradable: GaugeVec,
    uplink_latency_seconds: GaugeVec,
    uplink_speed_mbps: GaugeVec,
    uplink_max_speed_mbps: GaugeVec,
    uplink_uptime_seconds: GaugeVec,
    wan_receive_bytes_total: GaugeVec,
    wan_transmit_bytes_total: GaugeVec,
    wan_receive_packets_total: GaugeVec,
    wan_transmit_packets_total: GaugeVec,
    wan_receive_dropped_total: GaugeVec,
    wan_transmit_dropped_total: GaugeVec,
    wan_receive_errors_total: GaugeVec,
    wan_transmit_errors_total: GaugeVec,
    wan_receive_rate_bytes: GaugeVec,
    wan_transmit_rate_bytes: GaugeVec,
    wan_receive_broadcast_total: GaugeVec,
    wan_transmit_broadcast_total: GaugeVec,
    wan_receive_multicast_total: GaugeVec,
    wan_transmit_multicast_total: GaugeVec,
    lan_receive_bytes_total: GaugeVec,
    lan_transmit_bytes_total: GaugeVec,
    lan_receive_packets_total: GaugeVec,
    lan_transmit_packets_total: GaugeVec,
    lan_receive_dropped_total: GaugeVec,
    up: IntGaugeVec,
    fan_level: GaugeVec,
    max_power_total: GaugeVec,
    storage: GaugeVec,
    switch_receive_packets_total: GaugeVec,
    switch_receive_bytes_total: GaugeVec,
    switch_receive_errors_total: GaugeVec,
    switch_receive_dropped_total: GaugeVec,
    switch_receive_crypts_total: GaugeVec,
    switch_receive_frags_total: GaugeVec,
    switch_transmit_packets_total: GaugeVec,
    switch_transmit_bytes_total: GaugeVec,
    switch_transmit_errors_total: GaugeVec,
    switch_transmit_dropped_total: GaugeVec,
    switch_transmit_retries_total: GaugeVec,
    switch_receive_multicast_total: GaugeVec,
    switch_receive_broadcast_total: GaugeVec,
    switch_transmit_multicast_total: GaugeVec,
    switch_transmit_broadcast_total: GaugeVec,
    switch_bytes_total: GaugeVec,
    stat_wifi_transmit_dropped_total: GaugeVec,
    stat_receive_errors_total: GaugeVec,
    stat_receive_dropped_total: GaugeVec,
    stat_receive_frags_total: GaugeVec,
    stat_receive_crypts_total: GaugeVec,
    stat_transmit_packets_total: GaugeVec,
    stat_transmit_bytes_total: GaugeVec,
    stat_transmit_errors_total: GaugeVec,
    stat_transmit_dropped_total: GaugeVec,
    stat_retries_tx_total: GaugeVec,
    stat_receive_packets_total: GaugeVec,
    stat_receive_bytes_total: GaugeVec,
    stat_wifi_transmit_attempts_total: GaugeVec,
    stat_mac_filter_rejects_total: GaugeVec,
    outlet_cycle: GaugeVec,
    outlet_relay: GaugeVec,
    outlet_caps: GaugeVec,
    outlet_current: GaugeVec,
    outlet_power: GaugeVec,
    outlet_pf: GaugeVec,
    outlet_voltage: GaugeVec,
    outlet_ac: GaugeVec,
    power_source: GaugeVec,
    ups_batt_pct: GaugeVec,
    ups_batt_remain: GaugeVec,
    ups_charging: GaugeVec,
    ups_batt_mode: GaugeVec,
    ups_batt_avail: GaugeVec,
    ups_batt_ready: GaugeVec,
    ups_budget: GaugeVec,
    ups_output: GaugeVec,
    ups_pf: GaugeVec,
    ups_vout: GaugeVec,
    ups_iout: GaugeVec,
    ups_load: GaugeVec,
    ups_anomaly: GaugeVec,
    ups_cycle_recovery: GaugeVec,
    ci_operational: GaugeVec,
    ci_info: IntGaugeVec,
    count: IntGauge,
}

impl DeviceMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let ns = "unifi_device_";
        let m = Self {
            info: igauge(registry, &format!("{ns}info"), "Device information", INFO_LABELS)?,
            present: igauge(registry, &format!("{ns}present"), "Device present", LABELS)?,
            uptime_seconds: gauge(registry, &format!("{ns}uptime_seconds"), "Device uptime", LABELS)?,
            cpu_utilization_ratio: gauge(registry, &format!("{ns}cpu_utilization_ratio"), "CPU ratio", LABELS)?,
            memory_utilization_ratio: gauge(registry, &format!("{ns}memory_utilization_ratio"), "Memory ratio", LABELS)?,
            load_average_1: gauge(registry, &format!("{ns}load_average_1"), "Load 1m", LABELS)?,
            load_average_5: gauge(registry, &format!("{ns}load_average_5"), "Load 5m", LABELS)?,
            load_average_15: gauge(registry, &format!("{ns}load_average_15"), "Load 15m", LABELS)?,
            memory_used_bytes: gauge(registry, &format!("{ns}memory_used_bytes"), "Memory used", LABELS)?,
            memory_installed_bytes: gauge(registry, &format!("{ns}memory_installed_bytes"), "Memory installed", LABELS)?,
            memory_buffer_bytes: gauge(registry, &format!("{ns}memory_buffer_bytes"), "Memory buffer", LABELS)?,
            transmit_bytes_total: gauge(registry, &format!("{ns}transmit_bytes_total"), "TX bytes", LABELS)?,
            receive_bytes_total: gauge(registry, &format!("{ns}receive_bytes_total"), "RX bytes", LABELS)?,
            bytes_total: gauge(registry, &format!("{ns}bytes_total"), "Total bytes", LABELS)?,
            temperature_celsius: gauge(registry, &format!("{ns}temperature_celsius"), "Temperature", TEMP_LABELS)?,
            stations: gauge(registry, &format!("{ns}stations"), "Station count", STA_LABELS)?,
            upgradable: gauge(registry, &format!("{ns}upgradable"), "Firmware upgradable", LABELS)?,
            uplink_latency_seconds: gauge(registry, &format!("{ns}uplink_latency_seconds"), "Uplink latency", LABELS)?,
            uplink_speed_mbps: gauge(registry, &format!("{ns}uplink_speed_mbps"), "Uplink speed", LABELS)?,
            uplink_max_speed_mbps: gauge(registry, &format!("{ns}uplink_max_speed_mbps"), "Uplink max speed", LABELS)?,
            uplink_uptime_seconds: gauge(registry, &format!("{ns}uplink_uptime_seconds"), "Uplink uptime", LABELS)?,
            wan_receive_bytes_total: gauge(registry, &format!("{ns}wan_receive_bytes_total"), "WAN RX bytes", LABELS)?,
            wan_transmit_bytes_total: gauge(registry, &format!("{ns}wan_transmit_bytes_total"), "WAN TX bytes", LABELS)?,
            wan_receive_packets_total: gauge(registry, &format!("{ns}wan_receive_packets_total"), "WAN RX packets", LABELS)?,
            wan_transmit_packets_total: gauge(registry, &format!("{ns}wan_transmit_packets_total"), "WAN TX packets", LABELS)?,
            wan_receive_dropped_total: gauge(registry, &format!("{ns}wan_receive_dropped_total"), "WAN RX drops", LABELS)?,
            wan_transmit_dropped_total: gauge(registry, &format!("{ns}wan_transmit_dropped_total"), "WAN TX drops", LABELS)?,
            wan_receive_errors_total: gauge(registry, &format!("{ns}wan_receive_errors_total"), "WAN RX errors", LABELS)?,
            wan_transmit_errors_total: gauge(registry, &format!("{ns}wan_transmit_errors_total"), "WAN TX errors", LABELS)?,
            wan_receive_rate_bytes: gauge(registry, &format!("{ns}wan_receive_rate_bytes"), "WAN RX rate", LABELS)?,
            wan_transmit_rate_bytes: gauge(registry, &format!("{ns}wan_transmit_rate_bytes"), "WAN TX rate", LABELS)?,
            wan_receive_broadcast_total: gauge(registry, &format!("{ns}wan_receive_broadcast_total"), "WAN RX broadcast", LABELS)?,
            wan_transmit_broadcast_total: gauge(registry, &format!("{ns}wan_transmit_broadcast_total"), "WAN TX broadcast", LABELS)?,
            wan_receive_multicast_total: gauge(registry, &format!("{ns}wan_receive_multicast_total"), "WAN RX multicast", LABELS)?,
            wan_transmit_multicast_total: gauge(registry, &format!("{ns}wan_transmit_multicast_total"), "WAN TX multicast", LABELS)?,
            lan_receive_bytes_total: gauge(registry, &format!("{ns}lan_receive_bytes_total"), "LAN RX bytes", LABELS)?,
            lan_transmit_bytes_total: gauge(registry, &format!("{ns}lan_transmit_bytes_total"), "LAN TX bytes", LABELS)?,
            lan_receive_packets_total: gauge(registry, &format!("{ns}lan_receive_packets_total"), "LAN RX packets", LABELS)?,
            lan_transmit_packets_total: gauge(registry, &format!("{ns}lan_transmit_packets_total"), "LAN TX packets", LABELS)?,
            lan_receive_dropped_total: gauge(registry, &format!("{ns}lan_receive_dropped_total"), "LAN RX dropped", LABELS)?,
            up: igauge(registry, &format!("{ns}up"), "1 if the device is online", LABELS)?,
            fan_level: gauge(registry, &format!("{ns}fan_level"), "Fan level", LABELS)?,
            max_power_total: gauge(registry, &format!("{ns}max_power_total"), "Total max PoE power", LABELS)?,
            storage: gauge(registry, &format!("{ns}storage"), "Device storage", STORAGE_LABELS)?,
            switch_receive_packets_total: gauge(registry, &format!("{ns}switch_receive_packets_total"), "Switch RX packets", LABELS)?,
            switch_receive_bytes_total: gauge(registry, &format!("{ns}switch_receive_bytes_total"), "Switch RX bytes", LABELS)?,
            switch_receive_errors_total: gauge(registry, &format!("{ns}switch_receive_errors_total"), "Switch RX errors", LABELS)?,
            switch_receive_dropped_total: gauge(registry, &format!("{ns}switch_receive_dropped_total"), "Switch RX dropped", LABELS)?,
            switch_receive_crypts_total: gauge(registry, &format!("{ns}switch_receive_crypts_total"), "Switch RX crypts", LABELS)?,
            switch_receive_frags_total: gauge(registry, &format!("{ns}switch_receive_frags_total"), "Switch RX frags", LABELS)?,
            switch_transmit_packets_total: gauge(registry, &format!("{ns}switch_transmit_packets_total"), "Switch TX packets", LABELS)?,
            switch_transmit_bytes_total: gauge(registry, &format!("{ns}switch_transmit_bytes_total"), "Switch TX bytes", LABELS)?,
            switch_transmit_errors_total: gauge(registry, &format!("{ns}switch_transmit_errors_total"), "Switch TX errors", LABELS)?,
            switch_transmit_dropped_total: gauge(registry, &format!("{ns}switch_transmit_dropped_total"), "Switch TX dropped", LABELS)?,
            switch_transmit_retries_total: gauge(registry, &format!("{ns}switch_transmit_retries_total"), "Switch TX retries", LABELS)?,
            switch_receive_multicast_total: gauge(registry, &format!("{ns}switch_receive_multicast_total"), "Switch RX multicast", LABELS)?,
            switch_receive_broadcast_total: gauge(registry, &format!("{ns}switch_receive_broadcast_total"), "Switch RX broadcast", LABELS)?,
            switch_transmit_multicast_total: gauge(registry, &format!("{ns}switch_transmit_multicast_total"), "Switch TX multicast", LABELS)?,
            switch_transmit_broadcast_total: gauge(registry, &format!("{ns}switch_transmit_broadcast_total"), "Switch TX broadcast", LABELS)?,
            switch_bytes_total: gauge(registry, &format!("{ns}switch_bytes_total"), "Switch bytes", LABELS)?,
            stat_wifi_transmit_dropped_total: gauge(registry, &format!("{ns}stat_wifi_transmt_dropped_total"), "AP wifi TX dropped", STAT_LABELS)?,
            stat_receive_errors_total: gauge(registry, &format!("{ns}stat_receive_errors_total"), "AP RX errors", STAT_LABELS)?,
            stat_receive_dropped_total: gauge(registry, &format!("{ns}stat_receive_dropped_total"), "AP RX dropped", STAT_LABELS)?,
            stat_receive_frags_total: gauge(registry, &format!("{ns}stat_receive_frags_total"), "AP RX frags", STAT_LABELS)?,
            stat_receive_crypts_total: gauge(registry, &format!("{ns}stat_receive_crypts_total"), "AP RX crypts", STAT_LABELS)?,
            stat_transmit_packets_total: gauge(registry, &format!("{ns}stat_transmit_packets_total"), "AP TX packets", STAT_LABELS)?,
            stat_transmit_bytes_total: gauge(registry, &format!("{ns}stat_transmit_bytes_total"), "AP TX bytes", STAT_LABELS)?,
            stat_transmit_errors_total: gauge(registry, &format!("{ns}stat_transmit_errors_total"), "AP TX errors", STAT_LABELS)?,
            stat_transmit_dropped_total: gauge(registry, &format!("{ns}stat_transmit_dropped_total"), "AP TX dropped", STAT_LABELS)?,
            stat_retries_tx_total: gauge(registry, &format!("{ns}stat_retries_tx_total"), "AP TX retries", STAT_LABELS)?,
            stat_receive_packets_total: gauge(registry, &format!("{ns}stat_receive_packets_total"), "AP RX packets", STAT_LABELS)?,
            stat_receive_bytes_total: gauge(registry, &format!("{ns}stat_receive_bytes_total"), "AP RX bytes", STAT_LABELS)?,
            stat_wifi_transmit_attempts_total: gauge(registry, &format!("{ns}stat_wifi_transmit_attempts_total"), "AP wifi TX attempts", STAT_LABELS)?,
            stat_mac_filter_rejects_total: gauge(registry, &format!("{ns}stat_mac_filter_rejects_total"), "AP MAC filter rejects", STAT_LABELS)?,
            outlet_cycle: gauge(registry, &format!("{ns}outlet_cycle_enabled"), "PDU outlet cycle enabled", OUTLET)?,
            outlet_relay: gauge(registry, &format!("{ns}outlet_relay_state"), "PDU outlet relay state", OUTLET)?,
            outlet_caps: gauge(registry, &format!("{ns}outlet_outlet_caps"), "PDU outlet capabilities", OUTLET)?,
            outlet_current: gauge(registry, &format!("{ns}outlet_outlet_current"), "PDU outlet current", OUTLET)?,
            outlet_power: gauge(registry, &format!("{ns}outlet_outlet_power"), "PDU outlet power", OUTLET)?,
            outlet_pf: gauge(registry, &format!("{ns}outlet_outlet_power_factor"), "PDU outlet power factor", OUTLET)?,
            outlet_voltage: gauge(registry, &format!("{ns}outlet_outlet_voltage"), "PDU outlet voltage", OUTLET)?,
            outlet_ac: gauge(registry, &format!("{ns}outlet_ac_power_consumption"), "PDU AC power consumption", LABELS)?,
            power_source: gauge(registry, &format!("{ns}power_source"), "Device power source", LABELS)?,
            ups_batt_pct: gauge(registry, &format!("{ns}ups_battery_level_percent"), "UPS battery percent", UPS)?,
            ups_batt_remain: gauge(registry, &format!("{ns}ups_battery_time_remaining_seconds"), "UPS runtime remaining", UPS)?,
            ups_charging: gauge(registry, &format!("{ns}ups_battery_charging"), "UPS charging", UPS)?,
            ups_batt_mode: gauge(registry, &format!("{ns}ups_battery_mode"), "UPS on battery", UPS)?,
            ups_batt_avail: gauge(registry, &format!("{ns}ups_batteries_available"), "UPS batteries available", UPS)?,
            ups_batt_ready: gauge(registry, &format!("{ns}ups_batteries_ready"), "UPS batteries ready", UPS)?,
            ups_budget: gauge(registry, &format!("{ns}ups_power_budget_watts"), "UPS power budget", UPS)?,
            ups_output: gauge(registry, &format!("{ns}ups_power_output_watts"), "UPS power output", UPS)?,
            ups_pf: gauge(registry, &format!("{ns}ups_power_factor"), "UPS power factor", UPS)?,
            ups_vout: gauge(registry, &format!("{ns}ups_output_voltage"), "UPS output voltage", UPS)?,
            ups_iout: gauge(registry, &format!("{ns}ups_output_current_amps"), "UPS output current", UPS)?,
            ups_load: gauge(registry, &format!("{ns}ups_load_percent"), "UPS load percent", UPS)?,
            ups_anomaly: gauge(registry, &format!("{ns}ups_bms_anomaly_count"), "UPS BMS anomalies", UPS)?,
            ups_cycle_recovery: gauge(registry, &format!("{ns}ups_power_cycle_on_recovery_enabled"), "UPS cycle on AC recovery", UPS)?,
            ci_operational: gauge(registry, &format!("{ns}ci_state_operational"), "DOCSIS CI operational", LABELS)?,
            ci_info: igauge(registry, &format!("{ns}ci_state_info"), "DOCSIS CI state info", CI_INFO)?,
            count: IntGauge::with_opts(Opts::new("unifi_devices", "Adopted device count"))?,
        };
        registry.register(Box::new(m.count.clone()))?;
        Ok(m)
    }

    pub fn apply(&self, snap: &Snapshot) {
        self.reset();
        let source = snap.source.as_str();
        let mut total = 0i64;
        for site in &snap.sites {
            let site_name = site.site.name.as_deref().unwrap_or("");
            if !site.classic_devices.is_empty() {
                for d in &site.classic_devices {
                    total += 1;
                    emit_classic(self, d, site_name, source);
                }
            } else {
                for dws in &site.devices {
                    total += 1;
                    emit_integration(self, dws, site_name, source);
                }
            }
        }
        self.count.set(total);
    }

    fn reset(&self) {
        self.info.reset();
        self.present.reset();
        self.uptime_seconds.reset();
        self.cpu_utilization_ratio.reset();
        self.memory_utilization_ratio.reset();
        self.load_average_1.reset();
        self.load_average_5.reset();
        self.load_average_15.reset();
        self.memory_used_bytes.reset();
        self.memory_installed_bytes.reset();
        self.memory_buffer_bytes.reset();
        self.transmit_bytes_total.reset();
        self.receive_bytes_total.reset();
        self.bytes_total.reset();
        self.temperature_celsius.reset();
        self.stations.reset();
        self.upgradable.reset();
        self.uplink_latency_seconds.reset();
        self.uplink_speed_mbps.reset();
        self.uplink_max_speed_mbps.reset();
        self.uplink_uptime_seconds.reset();
        self.wan_receive_bytes_total.reset();
        self.wan_transmit_bytes_total.reset();
        self.wan_receive_packets_total.reset();
        self.wan_transmit_packets_total.reset();
        self.wan_receive_dropped_total.reset();
        self.wan_transmit_dropped_total.reset();
        self.wan_receive_errors_total.reset();
        self.wan_transmit_errors_total.reset();
        self.wan_receive_rate_bytes.reset();
        self.wan_transmit_rate_bytes.reset();
        self.wan_receive_broadcast_total.reset();
        self.wan_transmit_broadcast_total.reset();
        self.wan_receive_multicast_total.reset();
        self.wan_transmit_multicast_total.reset();
        self.lan_receive_bytes_total.reset();
        self.lan_transmit_bytes_total.reset();
        self.lan_receive_packets_total.reset();
        self.lan_transmit_packets_total.reset();
        self.lan_receive_dropped_total.reset();
        self.up.reset();
        self.fan_level.reset();
        self.max_power_total.reset();
        self.storage.reset();
        self.switch_receive_packets_total.reset();
        self.switch_receive_bytes_total.reset();
        self.switch_receive_errors_total.reset();
        self.switch_receive_dropped_total.reset();
        self.switch_receive_crypts_total.reset();
        self.switch_receive_frags_total.reset();
        self.switch_transmit_packets_total.reset();
        self.switch_transmit_bytes_total.reset();
        self.switch_transmit_errors_total.reset();
        self.switch_transmit_dropped_total.reset();
        self.switch_transmit_retries_total.reset();
        self.switch_receive_multicast_total.reset();
        self.switch_receive_broadcast_total.reset();
        self.switch_transmit_multicast_total.reset();
        self.switch_transmit_broadcast_total.reset();
        self.switch_bytes_total.reset();
        self.stat_wifi_transmit_dropped_total.reset();
        self.stat_receive_errors_total.reset();
        self.stat_receive_dropped_total.reset();
        self.stat_receive_frags_total.reset();
        self.stat_receive_crypts_total.reset();
        self.stat_transmit_packets_total.reset();
        self.stat_transmit_bytes_total.reset();
        self.stat_transmit_errors_total.reset();
        self.stat_transmit_dropped_total.reset();
        self.stat_retries_tx_total.reset();
        self.stat_receive_packets_total.reset();
        self.stat_receive_bytes_total.reset();
        self.stat_wifi_transmit_attempts_total.reset();
        self.stat_mac_filter_rejects_total.reset();
        self.outlet_cycle.reset();
        self.outlet_relay.reset();
        self.outlet_caps.reset();
        self.outlet_current.reset();
        self.outlet_power.reset();
        self.outlet_pf.reset();
        self.outlet_voltage.reset();
        self.outlet_ac.reset();
        self.power_source.reset();
        self.ups_batt_pct.reset();
        self.ups_batt_remain.reset();
        self.ups_charging.reset();
        self.ups_batt_mode.reset();
        self.ups_batt_avail.reset();
        self.ups_batt_ready.reset();
        self.ups_budget.reset();
        self.ups_output.reset();
        self.ups_pf.reset();
        self.ups_vout.reset();
        self.ups_iout.reset();
        self.ups_load.reset();
        self.ups_anomaly.reset();
        self.ups_cycle_recovery.reset();
        self.ci_operational.reset();
        self.ci_info.reset();
    }
}

fn emit_classic(m: &DeviceMetrics, d: &ClassicDevice, site_name: &str, source: &str) {
    let dtype = d.device_type.as_deref().unwrap_or("unknown");
    let name = d.display_name();
    let base = [dtype, site_name, name.as_str(), source];
    m.info
        .with_label_values(&[
            dtype,
            site_name,
            name.as_str(),
            source,
            d.version.as_deref().unwrap_or(""),
            d.model.as_deref().unwrap_or(""),
            d.serial.as_deref().unwrap_or(""),
            d.mac.as_deref().unwrap_or(""),
            d.ip.as_deref().unwrap_or(""),
            d._id.as_deref().unwrap_or(""),
        ])
        .set(1);
    m.present.with_label_values(&base).set(1);

    let uptime = d
        .system_stats
        .as_ref()
        .and_then(|s| s.uptime.get())
        .or_else(|| d.uptime.get());
    set(&m.uptime_seconds, &base, uptime);

    if let Some(sys) = &d.system_stats {
        set(
            &m.cpu_utilization_ratio,
            &base,
            sys.cpu.get().map(|v| if v > 1.0 { v / 100.0 } else { v }),
        );
        set(
            &m.memory_utilization_ratio,
            &base,
            sys.mem.get().map(|v| if v > 1.0 { v / 100.0 } else { v }),
        );
    }
    if let Some(ss) = &d.sys_stats {
        set(&m.load_average_1, &base, ss.loadavg_1.get());
        set(&m.load_average_5, &base, ss.loadavg_5.get());
        set(&m.load_average_15, &base, ss.loadavg_15.get());
        set(&m.memory_used_bytes, &base, ss.mem_used.get());
        set(&m.memory_installed_bytes, &base, ss.mem_total.get());
        set(&m.memory_buffer_bytes, &base, ss.mem_buffer.get());
    }
    set(&m.transmit_bytes_total, &base, d.tx_bytes.get());
    set(&m.receive_bytes_total, &base, d.rx_bytes.get());
    set(&m.bytes_total, &base, d.bytes.get());
    set(&m.upgradable, &base, d.upgradable.get());
    set(&m.stations, &[dtype, site_name, name.as_str(), source, "all"], d.num_sta.get());
    set(&m.stations, &[dtype, site_name, name.as_str(), source, "user"], d.user_num_sta.get());
    set(&m.stations, &[dtype, site_name, name.as_str(), source, "guest"], d.guest_num_sta.get());

    if let Some(temps) = &d.temperatures {
        for t in temps {
            set(
                &m.temperature_celsius,
                &[
                    dtype,
                    site_name,
                    name.as_str(),
                    source,
                    t.name.as_deref().unwrap_or(""),
                    t.temp_type.as_deref().unwrap_or(""),
                ],
                t.value.get(),
            );
        }
    }
    if let Some(up) = &d.uplink {
        set(
            &m.uplink_latency_seconds,
            &base,
            up.latency.get().map(|v| if v > 10.0 { v / 1000.0 } else { v }),
        );
        set(&m.uplink_speed_mbps, &base, up.speed.get());
        set(&m.uplink_max_speed_mbps, &base, up.max_speed.get());
        set(&m.uplink_uptime_seconds, &base, up.uptime.get());
    }

    set(&m.wan_receive_bytes_total, &base, d.gw_stat("wan-rx_bytes"));
    set(&m.wan_transmit_bytes_total, &base, d.gw_stat("wan-tx_bytes"));
    set(&m.wan_receive_packets_total, &base, d.gw_stat("wan-rx_packets"));
    set(&m.wan_transmit_packets_total, &base, d.gw_stat("wan-tx_packets"));
    set(&m.wan_receive_dropped_total, &base, d.gw_stat("wan-rx_dropped"));
    set(&m.wan_transmit_dropped_total, &base, d.gw_stat("wan-tx_dropped"));
    set(&m.wan_receive_errors_total, &base, d.gw_stat("wan-rx_errors"));
    set(&m.wan_transmit_errors_total, &base, d.gw_stat("wan-tx_errors"));
    set(&m.wan_receive_rate_bytes, &base, d.gw_stat("wan-rx_bytes-r"));
    set(&m.wan_transmit_rate_bytes, &base, d.gw_stat("wan-tx_bytes-r"));
    set(&m.wan_receive_broadcast_total, &base, d.gw_stat("wan-rx_broadcast"));
    set(&m.wan_transmit_broadcast_total, &base, d.gw_stat("wan-tx_broadcast"));
    set(&m.wan_receive_multicast_total, &base, d.gw_stat("wan-rx_multicast"));
    set(&m.wan_transmit_multicast_total, &base, d.gw_stat("wan-tx_multicast"));
    set(&m.lan_receive_bytes_total, &base, d.gw_stat("lan-rx_bytes"));
    set(&m.lan_transmit_bytes_total, &base, d.gw_stat("lan-tx_bytes"));
    set(&m.lan_receive_packets_total, &base, d.gw_stat("lan-rx_packets"));
    set(&m.lan_transmit_packets_total, &base, d.gw_stat("lan-tx_packets"));
    set(&m.lan_receive_dropped_total, &base, d.gw_stat("lan-rx_dropped"));

    m.up.with_label_values(&base).set(i64::from(d.online()));
    if d.has_fan.truthy() || d.fan_level.get().is_some() {
        set(&m.fan_level, &base, d.fan_level.get());
    }
    set(&m.max_power_total, &base, d.total_max_power.get());
    if d.has_temperature.truthy() {
        set(
            &m.temperature_celsius,
            &[dtype, site_name, name.as_str(), source, "general", "board"],
            d.general_temperature.get(),
        );
    }
    if let Some(storage) = &d.storage {
        for s in storage {
            let mount = s.mount_point.as_deref().unwrap_or("");
            let sname = s.name.as_deref().unwrap_or("");
            set(
                &m.storage,
                &[dtype, site_name, name.as_str(), source, mount, sname, "size"],
                s.size.get(),
            );
            set(
                &m.storage,
                &[dtype, site_name, name.as_str(), source, mount, sname, "used"],
                s.used.get(),
            );
        }
    }

    emit_switch(m, d, &base);
    emit_ap_stat(m, d, site_name, name.as_str(), source);
    emit_outlets(m, d, site_name, name.as_str(), source, &base);
}

fn emit_integration(
    m: &DeviceMetrics,
    dws: &crate::poll::DeviceWithStats,
    site_name: &str,
    source: &str,
) {
    let d = &dws.device;
    let dtype = "udm";
    let name = d.name.clone().unwrap_or_default();
    let base = [dtype, site_name, name.as_str(), source];
    m.info
        .with_label_values(&[
            dtype,
            site_name,
            name.as_str(),
            source,
            d.firmware_version.as_deref().unwrap_or(""),
            d.model.as_deref().unwrap_or(""),
            "",
            d.mac_address.as_deref().unwrap_or(""),
            d.ip_address.as_deref().unwrap_or(""),
            d.id.as_str(),
        ])
        .set(1);
    m.present.with_label_values(&base).set(1);
    let online = d
        .state
        .as_deref()
        .map(|s| {
            let s = s.to_ascii_lowercase();
            s == "online" || s == "connected"
        })
        .unwrap_or(false);
    m.up.with_label_values(&base).set(i64::from(online));
    if let Some(stats) = &dws.stats {
        set(
            &m.cpu_utilization_ratio,
            &base,
            stats.cpu_utilization_pct.map(|v| v / 100.0),
        );
        set(
            &m.memory_utilization_ratio,
            &base,
            stats.memory_utilization_pct.map(|v| v / 100.0),
        );
        set(&m.load_average_1, &base, stats.load_average1_min);
        set(&m.load_average_5, &base, stats.load_average5_min);
        set(&m.load_average_15, &base, stats.load_average15_min);
        set(
            &m.uptime_seconds,
            &base,
            stats.uptime_sec.map(|v| v as f64),
        );
        if let Some(up) = &stats.uplink {
            set(
                &m.wan_receive_rate_bytes,
                &base,
                up.rx_rate_bps.map(|v| v as f64 / 8.0),
            );
            set(
                &m.wan_transmit_rate_bytes,
                &base,
                up.tx_rate_bps.map(|v| v as f64 / 8.0),
            );
        }
    }
}

fn emit_switch(m: &DeviceMetrics, d: &ClassicDevice, base: &[&str]) {
    set(&m.switch_receive_packets_total, base, d.sw_stat("rx_packets"));
    set(&m.switch_receive_bytes_total, base, d.sw_stat("rx_bytes"));
    set(&m.switch_receive_errors_total, base, d.sw_stat("rx_errors"));
    set(&m.switch_receive_dropped_total, base, d.sw_stat("rx_dropped"));
    set(&m.switch_receive_crypts_total, base, d.sw_stat("rx_crypts"));
    set(&m.switch_receive_frags_total, base, d.sw_stat("rx_frags"));
    set(&m.switch_transmit_packets_total, base, d.sw_stat("tx_packets"));
    set(&m.switch_transmit_bytes_total, base, d.sw_stat("tx_bytes"));
    set(&m.switch_transmit_errors_total, base, d.sw_stat("tx_errors"));
    set(&m.switch_transmit_dropped_total, base, d.sw_stat("tx_dropped"));
    set(&m.switch_transmit_retries_total, base, d.sw_stat("tx_retries"));
    set(&m.switch_receive_multicast_total, base, d.sw_stat("rx_multicast"));
    set(&m.switch_receive_broadcast_total, base, d.sw_stat("rx_broadcast"));
    set(&m.switch_transmit_multicast_total, base, d.sw_stat("tx_multicast"));
    set(&m.switch_transmit_broadcast_total, base, d.sw_stat("tx_broadcast"));
    set(&m.switch_bytes_total, base, d.sw_stat("bytes"));
}

fn emit_ap_stat(m: &DeviceMetrics, d: &ClassicDevice, site_name: &str, name: &str, source: &str) {
    for kind in ["user", "guest"] {
        let labels = [kind, site_name, name, source];
        set(&m.stat_wifi_transmit_dropped_total, &labels, d.ap_stat(kind, "wifi_tx_dropped"));
        set(&m.stat_receive_errors_total, &labels, d.ap_stat(kind, "rx_errors"));
        set(&m.stat_receive_dropped_total, &labels, d.ap_stat(kind, "rx_dropped"));
        set(&m.stat_receive_frags_total, &labels, d.ap_stat(kind, "rx_frags"));
        set(&m.stat_receive_crypts_total, &labels, d.ap_stat(kind, "rx_crypts"));
        set(&m.stat_transmit_packets_total, &labels, d.ap_stat(kind, "tx_packets"));
        set(&m.stat_transmit_bytes_total, &labels, d.ap_stat(kind, "tx_bytes"));
        set(&m.stat_transmit_errors_total, &labels, d.ap_stat(kind, "tx_errors"));
        set(&m.stat_transmit_dropped_total, &labels, d.ap_stat(kind, "tx_dropped"));
        set(&m.stat_retries_tx_total, &labels, d.ap_stat(kind, "tx_retries"));
        set(&m.stat_receive_packets_total, &labels, d.ap_stat(kind, "rx_packets"));
        set(&m.stat_receive_bytes_total, &labels, d.ap_stat(kind, "rx_bytes"));
        set(&m.stat_wifi_transmit_attempts_total, &labels, d.ap_stat(kind, "wifi_tx_attempts"));
        set(&m.stat_mac_filter_rejects_total, &labels, d.ap_stat(kind, "mac_filter_rejections"));
    }
}

fn emit_outlets(m: &DeviceMetrics, d: &ClassicDevice, site_name: &str, name: &str, source: &str, base: &[&str]) {
    set(&m.outlet_ac, base, d.outlet_ac_power_consumption.get());
    set(&m.power_source, base, d.power_source.get());
    for o in &d.outlet_table {
        let idx = o.index.label();
        let oname = o.name.as_deref().unwrap_or("");
        let desc = format!("{name} Outlet {idx}");
        let labels = [desc.as_str(), idx.as_str(), oname, site_name, name, source];
        set(&m.outlet_cycle, &labels, o.cycle_enabled.get());
        set(&m.outlet_relay, &labels, o.relay_state.get());
        set(&m.outlet_caps, &labels, o.outlet_caps.get());
        set(&m.outlet_current, &labels, o.outlet_current.get());
        set(&m.outlet_power, &labels, o.outlet_power.get());
        set(&m.outlet_pf, &labels, o.outlet_power_factor.get());
        set(&m.outlet_voltage, &labels, o.outlet_voltage.get());
    }
    if let Some(vbms) = &d.vbms_table {
        let labels = [site_name, name, d.mac.as_deref().unwrap_or(""), source];
        set(&m.ups_anomaly, &labels, vbms.bms_run_anomaly.get());
        set(
            &m.ups_batt_mode,
            &labels,
            Some(if vbms.is_battery_mode.truthy() { 1.0 } else { 0.0 }),
        );
        set(
            &m.ups_cycle_recovery,
            &labels,
            Some(if d.outlet_power_cycle_on_ac_recovery_enabled.truthy() {
                1.0
            } else {
                0.0
            }),
        );
        if let Some(bp) = &vbms.batt_pool {
            set(&m.ups_batt_pct, &labels, bp.battery_level.get());
            set(&m.ups_batt_remain, &labels, bp.time_to_remain.get());
            set(
                &m.ups_charging,
                &labels,
                Some(if bp.is_charging.truthy() { 1.0 } else { 0.0 }),
            );
            set(&m.ups_batt_avail, &labels, bp.batt_available_cnt.get());
            set(&m.ups_batt_ready, &labels, bp.ready_cnt.get());
            set(&m.ups_budget, &labels, bp.device_total_power_budget.get());
            set(&m.ups_output, &labels, bp.device_total_power_output.get());
            set(&m.ups_pf, &labels, bp.device_total_power_factor.get());
            set(&m.ups_vout, &labels, bp.device_output_voltage.get());
            set(&m.ups_iout, &labels, bp.device_output_current.get());
            let load = match (bp.device_total_power_output.get(), bp.device_total_power_budget.get()) {
                (Some(out), Some(budget)) if budget > 0.0 => Some(out / budget * 100.0),
                _ => None,
            };
            set(&m.ups_load, &labels, load);
        }
    }
    if let Some(ci) = &d.ci_state_table {
        let operational = if ci.ci_state.as_deref() == Some("Operational") {
            1.0
        } else {
            0.0
        };
        set(&m.ci_operational, base, Some(operational));
        m.ci_info
            .with_label_values(&[
                d.device_type.as_deref().unwrap_or("unknown"),
                site_name,
                name,
                source,
                ci.ci_state.as_deref().unwrap_or(""),
                ci.ci_sw_dl_status.as_deref().unwrap_or(""),
                ci.ci_mac.as_deref().unwrap_or(""),
                ci.ci_version.as_deref().unwrap_or(""),
                ci.ci_mode.as_deref().unwrap_or(""),
            ])
            .set(1);
    }
}
