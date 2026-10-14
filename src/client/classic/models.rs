//! Classic controller JSON models. UniFi often encodes numbers as strings.

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Envelope<T> {
    #[serde(default)]
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Flex(pub Option<f64>);

impl Flex {
    pub fn get(self) -> Option<f64> {
        self.0
    }

    pub fn truthy(self) -> bool {
        self.get().map(|v| v != 0.0).unwrap_or(false)
    }

    pub fn label(self) -> String {
        match self.get() {
            Some(v) if v.fract() == 0.0 && v.abs() < 1e15 => format!("{}", v as i64),
            Some(v) => format!("{v}"),
            None => String::new(),
        }
    }
}

/// String-or-number JSON field (DPI category IDs, etc.).
#[derive(Debug, Clone, Default)]
pub struct FlexStr(pub Option<String>);

impl FlexStr {
    pub fn as_str(&self) -> &str {
        self.0.as_deref().unwrap_or("")
    }
}

impl<'de> Deserialize<'de> for FlexStr {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = Value::deserialize(deserializer)?;
        Ok(FlexStr(value_string(&v)))
    }
}

fn value_string(v: &Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

impl<'de> Deserialize<'de> for Flex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = Value::deserialize(deserializer)?;
        Ok(Flex(value_f64(&v)))
    }
}

pub fn value_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Null => None,
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        _ => None,
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ClassicClient {
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub oui: Option<String>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub essid: Option<String>,
    #[serde(default)]
    pub bssid: Option<String>,
    #[serde(default)]
    pub radio: Option<String>,
    #[serde(default, rename = "radio_name")]
    pub radio_name: Option<String>,
    #[serde(default, rename = "radio_proto")]
    pub radio_proto: Option<String>,
    #[serde(default, rename = "radio_desc")]
    pub radio_desc: Option<String>,
    #[serde(default)]
    pub channel: Flex,
    #[serde(default, rename = "is_wired")]
    pub is_wired: Flex,
    #[serde(default)]
    pub vlan: Flex,
    #[serde(default, rename = "sw_mac")]
    pub sw_mac: Option<String>,
    #[serde(default, rename = "sw_port")]
    pub sw_port: Flex,
    #[serde(default, rename = "ap_mac")]
    pub ap_mac: Option<String>,
    #[serde(default, rename = "gw_mac")]
    pub gw_mac: Option<String>,
    #[serde(default)]
    pub rssi: Flex,
    #[serde(default)]
    pub signal: Flex,
    #[serde(default)]
    pub noise: Flex,
    #[serde(default)]
    pub ccq: Flex,
    #[serde(default)]
    pub satisfaction: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_packets")]
    pub tx_packets: Flex,
    #[serde(default, rename = "rx_packets")]
    pub rx_packets: Flex,
    #[serde(default, rename = "tx_retries")]
    pub tx_retries: Flex,
    #[serde(default, rename = "tx_rate")]
    pub tx_rate: Flex,
    #[serde(default, rename = "rx_rate")]
    pub rx_rate: Flex,
    #[serde(default, rename = "tx_bytes-r")]
    pub tx_bytes_r: Flex,
    #[serde(default, rename = "rx_bytes-r")]
    pub rx_bytes_r: Flex,
    #[serde(default, rename = "bytes-r")]
    pub bytes_r: Flex,
    #[serde(default)]
    pub uptime: Flex,
    #[serde(default, rename = "roam_count")]
    pub roam_count: Flex,
    #[serde(default, rename = "tx_mcs")]
    pub tx_mcs: Flex,
    #[serde(default, rename = "rx_mcs")]
    pub rx_mcs: Flex,
    #[serde(default, rename = "tx_nss")]
    pub tx_nss: Flex,
    #[serde(default, rename = "rx_nss")]
    pub rx_nss: Flex,
    #[serde(default, rename = "wifi_tx_attempts")]
    pub wifi_tx_attempts: Flex,
    #[serde(default)]
    pub anomalies: Flex,
    #[serde(default, rename = "wired-tx_bytes")]
    pub wired_tx_bytes: Flex,
    #[serde(default, rename = "wired-rx_bytes")]
    pub wired_rx_bytes: Flex,
    #[serde(default, rename = "wired-tx_packets")]
    pub wired_tx_packets: Flex,
    #[serde(default, rename = "wired-rx_packets")]
    pub wired_rx_packets: Flex,
    #[serde(default, rename = "wired-tx_bytes-r")]
    pub wired_tx_bytes_r: Flex,
    #[serde(default, rename = "wired-rx_bytes-r")]
    pub wired_rx_bytes_r: Flex,
    #[serde(default, rename = "tx_power")]
    pub tx_power: Flex,
}

impl ClassicClient {
    pub fn display_name(&self) -> String {
        self.name
            .clone()
            .filter(|s| !s.is_empty())
            .or_else(|| self.hostname.clone().filter(|s| !s.is_empty()))
            .or_else(|| self.mac.clone())
            .unwrap_or_default()
    }

    pub fn wired(&self) -> bool {
        self.is_wired.get().map(|v| v != 0.0).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ClassicDevice {
    #[serde(default)]
    pub _id: Option<String>,
    #[serde(default, rename = "type")]
    pub device_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub state: Flex,
    #[serde(default)]
    pub adopted: Flex,
    #[serde(default)]
    pub uptime: Flex,
    #[serde(default, rename = "upgradable")]
    pub upgradable: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default)]
    pub bytes: Flex,
    #[serde(default, rename = "num_sta")]
    pub num_sta: Flex,
    #[serde(default, rename = "user-num_sta")]
    pub user_num_sta: Flex,
    #[serde(default, rename = "guest-num_sta")]
    pub guest_num_sta: Flex,
    #[serde(default, rename = "sys_stats")]
    pub sys_stats: Option<SysStats>,
    #[serde(default, rename = "system-stats")]
    pub system_stats: Option<SystemStats>,
    #[serde(default)]
    pub uplink: Option<Uplink>,
    #[serde(default)]
    pub temperatures: Option<Vec<Temperature>>,
    #[serde(default)]
    pub storage: Option<Vec<Storage>>,
    #[serde(default, rename = "general_temperature")]
    pub general_temperature: Flex,
    #[serde(default, rename = "has_temperature")]
    pub has_temperature: Flex,
    #[serde(default, rename = "has_fan")]
    pub has_fan: Flex,
    #[serde(default, rename = "fan_level")]
    pub fan_level: Flex,
    #[serde(default, rename = "total_max_power")]
    pub total_max_power: Flex,
    #[serde(default, rename = "port_table")]
    pub port_table: Vec<Port>,
    #[serde(default, rename = "radio_table")]
    pub radio_table: Vec<Radio>,
    #[serde(default, rename = "radio_table_stats")]
    pub radio_table_stats: Vec<Radio>,
    #[serde(default, rename = "vap_table")]
    pub vap_table: Vec<Vap>,
    #[serde(default, rename = "outlet_table")]
    pub outlet_table: Vec<Outlet>,
    #[serde(default, rename = "vbms_table")]
    pub vbms_table: Option<VbmsTable>,
    #[serde(default, rename = "ci_state_table")]
    pub ci_state_table: Option<CiStateTable>,
    #[serde(default, rename = "outlet_ac_power_consumption")]
    pub outlet_ac_power_consumption: Flex,
    #[serde(default, rename = "power_source")]
    pub power_source: Flex,
    #[serde(default, rename = "outlet_power_cycle_on_ac_recovery_enabled")]
    pub outlet_power_cycle_on_ac_recovery_enabled: Flex,
    #[serde(default)]
    pub wan1: Option<WanIface>,
    #[serde(default)]
    pub wan2: Option<WanIface>,
    #[serde(default)]
    pub stat: Option<Value>,
}

impl ClassicDevice {
    pub fn display_name(&self) -> String {
        self.name
            .clone()
            .filter(|s| !s.is_empty())
            .or_else(|| self.mac.clone())
            .unwrap_or_default()
    }

    pub fn online(&self) -> bool {
        matches!(self.state.get(), Some(v) if (v - 1.0).abs() < f64::EPSILON)
            || self.adopted.get().map(|v| v != 0.0).unwrap_or(false)
                && self.state.get().unwrap_or(1.0) != 0.0
    }

    pub fn gw_stat(&self, key: &str) -> Option<f64> {
        self.nested_stat("gw", key)
    }

    pub fn nested_stat(&self, table: &str, key: &str) -> Option<f64> {
        self.stat.as_ref()?.get(table)?.get(key).and_then(value_f64)
    }

    pub fn ap_stat(&self, kind: &str, key: &str) -> Option<f64> {
        self.nested_stat("ap", &format!("{kind}-{key}"))
    }

    pub fn sw_stat(&self, key: &str) -> Option<f64> {
        self.nested_stat("sw", key)
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysStats {
    #[serde(default, rename = "loadavg_1")]
    pub loadavg_1: Flex,
    #[serde(default, rename = "loadavg_5")]
    pub loadavg_5: Flex,
    #[serde(default, rename = "loadavg_15")]
    pub loadavg_15: Flex,
    #[serde(default, rename = "mem_used")]
    pub mem_used: Flex,
    #[serde(default, rename = "mem_buffer")]
    pub mem_buffer: Flex,
    #[serde(default, rename = "mem_total")]
    pub mem_total: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SystemStats {
    #[serde(default)]
    pub cpu: Flex,
    #[serde(default)]
    pub mem: Flex,
    #[serde(default)]
    pub uptime: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Uplink {
    #[serde(default)]
    pub latency: Flex,
    #[serde(default)]
    pub speed: Flex,
    #[serde(default, rename = "max_speed")]
    pub max_speed: Flex,
    #[serde(default)]
    pub uptime: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default)]
    pub up: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Temperature {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub temp_type: Option<String>,
    #[serde(default)]
    pub value: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Storage {
    #[serde(default, rename = "mount_point")]
    pub mount_point: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub size: Flex,
    #[serde(default)]
    pub used: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct WanIface {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub up: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default, rename = "rx_packets")]
    pub rx_packets: Flex,
    #[serde(default, rename = "tx_packets")]
    pub tx_packets: Flex,
    #[serde(default, rename = "rx_dropped")]
    pub rx_dropped: Flex,
    #[serde(default, rename = "tx_dropped")]
    pub tx_dropped: Flex,
    #[serde(default, rename = "rx_errors")]
    pub rx_errors: Flex,
    #[serde(default, rename = "tx_errors")]
    pub tx_errors: Flex,
    #[serde(default, rename = "rx_broadcast")]
    pub rx_broadcast: Flex,
    #[serde(default, rename = "tx_broadcast")]
    pub tx_broadcast: Flex,
    #[serde(default, rename = "rx_multicast")]
    pub rx_multicast: Flex,
    #[serde(default, rename = "tx_multicast")]
    pub tx_multicast: Flex,
    #[serde(default, rename = "rx_bytes-r")]
    pub rx_bytes_r: Flex,
    #[serde(default, rename = "tx_bytes-r")]
    pub tx_bytes_r: Flex,
    #[serde(default)]
    pub speed: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Port {
    #[serde(default, rename = "port_idx")]
    pub port_idx: Flex,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub up: Flex,
    #[serde(default)]
    pub enable: Flex,
    #[serde(default)]
    pub speed: Flex,
    #[serde(default)]
    pub satisfaction: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default, rename = "rx_bytes-r")]
    pub rx_bytes_r: Flex,
    #[serde(default, rename = "tx_bytes-r")]
    pub tx_bytes_r: Flex,
    #[serde(default, rename = "rx_packets")]
    pub rx_packets: Flex,
    #[serde(default, rename = "tx_packets")]
    pub tx_packets: Flex,
    #[serde(default, rename = "rx_dropped")]
    pub rx_dropped: Flex,
    #[serde(default, rename = "tx_dropped")]
    pub tx_dropped: Flex,
    #[serde(default, rename = "rx_errors")]
    pub rx_errors: Flex,
    #[serde(default, rename = "tx_errors")]
    pub tx_errors: Flex,
    #[serde(default, rename = "rx_broadcast")]
    pub rx_broadcast: Flex,
    #[serde(default, rename = "tx_broadcast")]
    pub tx_broadcast: Flex,
    #[serde(default, rename = "rx_multicast")]
    pub rx_multicast: Flex,
    #[serde(default, rename = "tx_multicast")]
    pub tx_multicast: Flex,
    #[serde(default, rename = "poe_enable")]
    pub poe_enable: Flex,
    #[serde(default, rename = "port_poe")]
    pub port_poe: Flex,
    #[serde(default, rename = "poe_current")]
    pub poe_current: Flex,
    #[serde(default, rename = "poe_power")]
    pub poe_power: Flex,
    #[serde(default, rename = "poe_voltage")]
    pub poe_voltage: Flex,
    #[serde(default, rename = "sfp_found")]
    pub sfp_found: Flex,
    #[serde(default, rename = "sfp_part")]
    pub sfp_part: Option<String>,
    #[serde(default, rename = "sfp_vendor")]
    pub sfp_vendor: Option<String>,
    #[serde(default, rename = "sfp_serial")]
    pub sfp_serial: Option<String>,
    #[serde(default, rename = "sfp_compliance")]
    pub sfp_compliance: Option<String>,
    #[serde(default, rename = "sfp_current")]
    pub sfp_current: Flex,
    #[serde(default, rename = "sfp_voltage")]
    pub sfp_voltage: Flex,
    #[serde(default, rename = "sfp_temperature")]
    pub sfp_temperature: Flex,
    #[serde(default, rename = "sfp_rxpower")]
    pub sfp_rxpower: Flex,
    #[serde(default, rename = "sfp_txpower")]
    pub sfp_txpower: Flex,
}

impl Port {
    pub fn alive(&self) -> bool {
        let up = self.up.get().map(|v| v != 0.0).unwrap_or(true);
        let enable = self.enable.get().map(|v| v != 0.0).unwrap_or(true);
        let poe = self.poe_power.get().unwrap_or(0.0) > 0.0;
        (up && enable) || poe
    }

    pub fn idx_label(&self) -> String {
        self.port_idx.label()
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Radio {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub radio: Option<String>,
    #[serde(default, rename = "current_antenna_gain")]
    pub current_antenna_gain: Flex,
    #[serde(default)]
    pub ht: Flex,
    #[serde(default, rename = "max_txpower")]
    pub max_txpower: Flex,
    #[serde(default, rename = "min_txpower")]
    pub min_txpower: Flex,
    #[serde(default)]
    pub nss: Flex,
    #[serde(default, rename = "radio_caps")]
    pub radio_caps: Flex,
    #[serde(default, rename = "tx_power")]
    pub tx_power: Flex,
    #[serde(default, rename = "ast_be_xmit")]
    pub ast_be_xmit: Flex,
    #[serde(default)]
    pub channel: Flex,
    #[serde(default, rename = "cu_self_rx")]
    pub cu_self_rx: Flex,
    #[serde(default, rename = "cu_self_tx")]
    pub cu_self_tx: Flex,
    #[serde(default, rename = "cu_total")]
    pub cu_total: Flex,
    #[serde(default)]
    pub extchannel: Flex,
    #[serde(default)]
    pub gain: Flex,
    #[serde(default, rename = "num_sta")]
    pub num_sta: Flex,
    #[serde(default, rename = "user-num_sta")]
    pub user_num_sta: Flex,
    #[serde(default, rename = "guest-num_sta")]
    pub guest_num_sta: Flex,
    #[serde(default, rename = "tx_packets")]
    pub tx_packets: Flex,
    #[serde(default, rename = "tx_retries")]
    pub tx_retries: Flex,
}

impl Radio {
    pub fn band(&self) -> &'static str {
        radio_band(self.radio.as_deref().unwrap_or(""))
    }
}

pub fn radio_band(radio: &str) -> &'static str {
    match radio {
        "ng" => "2.4",
        "na" => "5",
        "6e" => "6",
        _ => "",
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Vap {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub bssid: Option<String>,
    #[serde(default)]
    pub radio: Option<String>,
    #[serde(default, rename = "radio_name")]
    pub radio_name: Option<String>,
    #[serde(default)]
    pub essid: Option<String>,
    #[serde(default)]
    pub usage: Option<String>,
    #[serde(default)]
    pub up: Flex,
    #[serde(default)]
    pub ccq: Flex,
    #[serde(default, rename = "mac_filter_rejections")]
    pub mac_filter_rejections: Flex,
    #[serde(default, rename = "num_satisfaction_sta")]
    pub num_satisfaction_sta: Flex,
    #[serde(default, rename = "num_sta")]
    pub num_sta: Flex,
    #[serde(default, rename = "avg_client_signal")]
    pub avg_client_signal: Flex,
    #[serde(default)]
    pub satisfaction: Flex,
    #[serde(default, rename = "satisfaction_now")]
    pub satisfaction_now: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default, rename = "rx_packets")]
    pub rx_packets: Flex,
    #[serde(default, rename = "tx_packets")]
    pub tx_packets: Flex,
    #[serde(default, rename = "rx_dropped")]
    pub rx_dropped: Flex,
    #[serde(default, rename = "tx_dropped")]
    pub tx_dropped: Flex,
    #[serde(default, rename = "rx_errors")]
    pub rx_errors: Flex,
    #[serde(default, rename = "tx_errors")]
    pub tx_errors: Flex,
    #[serde(default, rename = "rx_crypts")]
    pub rx_crypts: Flex,
    #[serde(default, rename = "rx_frags")]
    pub rx_frags: Flex,
    #[serde(default, rename = "rx_nwids")]
    pub rx_nwids: Flex,
    #[serde(default, rename = "tx_retries")]
    pub tx_retries: Flex,
    #[serde(default, rename = "tx_combined_retries")]
    pub tx_combined_retries: Flex,
    #[serde(default, rename = "tx_power")]
    pub tx_power: Flex,
    #[serde(default, rename = "tx_rts_retries")]
    pub tx_rts_retries: Flex,
    #[serde(default, rename = "tx_total")]
    pub tx_total: Flex,
    #[serde(default, rename = "dns_avg_latency")]
    pub dns_avg_latency: Flex,
    #[serde(default, rename = "tx_data_mpdu_bytes")]
    pub tx_data_mpdu_bytes: Flex,
    #[serde(default, rename = "tx_tcp_stats")]
    pub tx_tcp_stats: Option<TcpStats>,
    #[serde(default, rename = "rx_tcp_stats")]
    pub rx_tcp_stats: Option<TcpStats>,
    #[serde(default, rename = "wifi_tx_latency_mov")]
    pub wifi_tx_latency_mov: Option<LatencyMov>,
}

impl Vap {
    pub fn is_up(&self) -> bool {
        self.up.get().map(|v| v != 0.0).unwrap_or(true)
    }

    pub fn band(&self) -> &'static str {
        radio_band(self.radio.as_deref().unwrap_or(""))
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TcpStats {
    #[serde(default)]
    pub goodbytes: Flex,
    #[serde(default, rename = "lat_avg")]
    pub lat_avg: Flex,
    #[serde(default, rename = "lat_max")]
    pub lat_max: Flex,
    #[serde(default, rename = "lat_min")]
    pub lat_min: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LatencyMov {
    #[serde(default)]
    pub avg: Flex,
    #[serde(default)]
    pub max: Flex,
    #[serde(default)]
    pub min: Flex,
    #[serde(default)]
    pub total: Flex,
    #[serde(default, rename = "total_count")]
    pub total_count: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Outlet {
    #[serde(default)]
    pub index: Flex,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "cycle_enabled")]
    pub cycle_enabled: Flex,
    #[serde(default, rename = "relay_state")]
    pub relay_state: Flex,
    #[serde(default, rename = "outlet_caps")]
    pub outlet_caps: Flex,
    #[serde(default, rename = "outlet_current")]
    pub outlet_current: Flex,
    #[serde(default, rename = "outlet_power")]
    pub outlet_power: Flex,
    #[serde(default, rename = "outlet_power_factor")]
    pub outlet_power_factor: Flex,
    #[serde(default, rename = "outlet_voltage")]
    pub outlet_voltage: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct VbmsTable {
    #[serde(default, rename = "bms_run_anomaly")]
    pub bms_run_anomaly: Flex,
    #[serde(default, rename = "is_battery_mode")]
    pub is_battery_mode: Flex,
    #[serde(default, rename = "batt_pool")]
    pub batt_pool: Option<BattPool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BattPool {
    #[serde(default, rename = "battery_level")]
    pub battery_level: Flex,
    #[serde(default, rename = "time_to_remain")]
    pub time_to_remain: Flex,
    #[serde(default, rename = "is_charging")]
    pub is_charging: Flex,
    #[serde(default, rename = "batt_available_cnt")]
    pub batt_available_cnt: Flex,
    #[serde(default, rename = "ready_cnt")]
    pub ready_cnt: Flex,
    #[serde(default, rename = "device_total_power_budget")]
    pub device_total_power_budget: Flex,
    #[serde(default, rename = "device_total_power_output")]
    pub device_total_power_output: Flex,
    #[serde(default, rename = "device_total_power_factor")]
    pub device_total_power_factor: Flex,
    #[serde(default, rename = "device_output_voltage")]
    pub device_output_voltage: Flex,
    #[serde(default, rename = "device_output_current")]
    pub device_output_current: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CiStateTable {
    #[serde(default, rename = "ci_state")]
    pub ci_state: Option<String>,
    #[serde(default, rename = "ci_sw_dl_status")]
    pub ci_sw_dl_status: Option<String>,
    #[serde(default, rename = "ci_mac")]
    pub ci_mac: Option<String>,
    #[serde(default, rename = "ci_version")]
    pub ci_version: Option<String>,
    #[serde(default, rename = "ci_mode")]
    pub ci_mode: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct HealthSubsystem {
    #[serde(default)]
    pub subsystem: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "num_user")]
    pub num_user: Flex,
    #[serde(default, rename = "num_guest")]
    pub num_guest: Flex,
    #[serde(default, rename = "num_sta")]
    pub num_sta: Flex,
    #[serde(default, rename = "num_iot")]
    pub num_iot: Flex,
    #[serde(default, rename = "num_ap")]
    pub num_ap: Flex,
    #[serde(default, rename = "num_adopted")]
    pub num_adopted: Flex,
    #[serde(default, rename = "num_disabled")]
    pub num_disabled: Flex,
    #[serde(default, rename = "num_disconnected")]
    pub num_disconnected: Flex,
    #[serde(default, rename = "num_pending")]
    pub num_pending: Flex,
    #[serde(default, rename = "num_gw")]
    pub num_gw: Flex,
    #[serde(default, rename = "num_sw")]
    pub num_sw: Flex,
    #[serde(default)]
    pub latency: Flex,
    #[serde(default)]
    pub uptime: Flex,
    #[serde(default)]
    pub drops: Flex,
    #[serde(default, rename = "xput_up")]
    pub xput_up: Flex,
    #[serde(default, rename = "xput_down")]
    pub xput_down: Flex,
    #[serde(default, rename = "speedtest_ping")]
    pub speedtest_ping: Flex,
    #[serde(default, rename = "tx_bytes-r")]
    pub tx_bytes_r: Flex,
    #[serde(default, rename = "rx_bytes-r")]
    pub rx_bytes_r: Flex,
    #[serde(default, rename = "remote_user_num_active")]
    pub remote_user_num_active: Flex,
    #[serde(default, rename = "remote_user_num_inactive")]
    pub remote_user_num_inactive: Flex,
    #[serde(default, rename = "remote_user_rx_bytes")]
    pub remote_user_rx_bytes: Flex,
    #[serde(default, rename = "remote_user_tx_bytes")]
    pub remote_user_tx_bytes: Flex,
    #[serde(default, rename = "remote_user_rx_packets")]
    pub remote_user_rx_packets: Flex,
    #[serde(default, rename = "remote_user_tx_packets")]
    pub remote_user_tx_packets: Flex,
    #[serde(default, rename = "site_to_site_enabled")]
    pub site_to_site_enabled: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DpiEntry {
    #[serde(default)]
    pub cat: FlexStr,
    #[serde(default)]
    pub app: FlexStr,
    #[serde(default)]
    pub category: FlexStr,
    #[serde(default)]
    pub application: FlexStr,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_packets")]
    pub tx_packets: Flex,
    #[serde(default, rename = "rx_packets")]
    pub rx_packets: Flex,
}

impl DpiEntry {
    pub fn category(&self) -> &str {
        let c = self.category.as_str();
        if c.is_empty() {
            self.cat.as_str()
        } else {
            c
        }
    }

    pub fn application(&self) -> &str {
        let a = self.application.as_str();
        if a.is_empty() {
            self.app.as_str()
        } else {
            a
        }
    }
}

/// `/stat/dpi` and `/stat/stadpi` rows are either nested `{by_app:[...]}` or flat.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DpiRow {
    Nested(DpiTable),
    Flat(DpiEntry),
}

impl Default for DpiRow {
    fn default() -> Self {
        Self::Nested(DpiTable::default())
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DpiTable {
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    pub by_app: Vec<DpiEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientDpi {
    pub mac: Option<String>,
    pub name: Option<String>,
    pub entries: Vec<DpiEntry>,
}

pub fn flatten_dpi(rows: Vec<DpiRow>) -> Vec<DpiEntry> {
    let mut out = Vec::new();
    for row in rows {
        match row {
            DpiRow::Nested(t) => out.extend(t.by_app),
            DpiRow::Flat(e) => out.push(e),
        }
    }
    out
}

pub fn flatten_stadpi(rows: Vec<DpiRow>) -> Vec<ClientDpi> {
    let mut out = Vec::new();
    for row in rows {
        match row {
            DpiRow::Nested(t) => {
                if t.by_app.is_empty() {
                    continue;
                }
                out.push(ClientDpi {
                    mac: t.mac,
                    name: t.name.or(t.hostname),
                    entries: t.by_app,
                });
            }
            DpiRow::Flat(e) => out.push(ClientDpi {
                mac: None,
                name: None,
                entries: vec![e],
            }),
        }
    }
    out
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RogueAp {
    #[serde(default)]
    pub essid: Option<String>,
    #[serde(default)]
    pub bssid: Option<String>,
    #[serde(default)]
    pub security: Option<String>,
    #[serde(default)]
    pub oui: Option<String>,
    #[serde(default)]
    pub band: Option<String>,
    #[serde(default, rename = "ap_mac")]
    pub ap_mac: Option<String>,
    #[serde(default)]
    pub radio: Option<String>,
    #[serde(default, rename = "radio_name")]
    pub radio_name: Option<String>,
    #[serde(default)]
    pub rssi: Flex,
    #[serde(default)]
    pub age: Flex,
    #[serde(default)]
    pub channel: Flex,
    #[serde(default)]
    pub noise: Flex,
    #[serde(default)]
    pub signal: Flex,
    #[serde(default)]
    pub bw: Flex,
    #[serde(default, rename = "center_freq")]
    pub center_freq: Flex,
    #[serde(default)]
    pub freq: Flex,
    #[serde(default, rename = "rssi_age")]
    pub rssi_age: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SpeedtestStatus {
    #[serde(default)]
    pub latency: Flex,
    #[serde(default)]
    pub rundate: Flex,
    #[serde(default)]
    pub runtime: Flex,
    #[serde(default, rename = "xput_download")]
    pub xput_download: Flex,
    #[serde(default, rename = "xput_upload")]
    pub xput_upload: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Sysinfo {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default, rename = "ubnt_device_type")]
    pub ubnt_device_type: Option<String>,
    #[serde(default, rename = "console_display_version")]
    pub console_display_version: Option<String>,
    #[serde(default, rename = "previous_version")]
    pub previous_version: Option<String>,
    #[serde(default)]
    pub uptime: Flex,
    #[serde(default, rename = "update_available")]
    pub update_available: Flex,
    #[serde(default, rename = "update_downloaded")]
    pub update_downloaded: Flex,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_port_radio_vap_and_nested_dpi() {
        let device: ClassicDevice = serde_json::from_value(serde_json::json!({
            "type": "udm",
            "name": "UDM",
            "mac": "aa:bb:cc:dd:ee:ff",
            "state": 1,
            "port_table": [{
                "port_idx": 1,
                "name": "Port 1",
                "up": true,
                "enable": true,
                "rx_bytes": 100,
                "tx_bytes": 200,
                "speed": 1000
            }],
            "radio_table": [{"name": "wifi0", "radio": "ng", "channel": 6}],
            "radio_table_stats": [{"name": "wifi0", "cu_total": 12, "tx_retries": 3}],
            "vap_table": [{"name": "ath0", "essid": "Home", "radio": "ng", "up": true, "rx_bytes": 9, "tx_tcp_stats": {"goodbytes": 4, "lat_avg": 12}}],
            "outlet_table": [{"index": 1, "name": "Camera", "relay_state": true, "outlet_power": 8.5}],
            "ci_state_table": {"ci_state": "Operational"},
            "stat": {"ap": {"user-rx_bytes": 1, "guest-tx_bytes": 2}, "sw": {"rx_bytes": 3}}
        }))
        .unwrap();
        assert_eq!(device.port_table.len(), 1);
        assert!(device.port_table[0].alive());
        assert_eq!(device.radio_table[0].band(), "2.4");
        assert_eq!(device.ap_stat("user", "rx_bytes"), Some(1.0));
        assert_eq!(device.sw_stat("rx_bytes"), Some(3.0));
        assert_eq!(device.outlet_table[0].outlet_power.get(), Some(8.5));
        assert_eq!(
            device.ci_state_table.as_ref().and_then(|c| c.ci_state.as_deref()),
            Some("Operational")
        );
        assert_eq!(
            device.vap_table[0]
                .tx_tcp_stats
                .as_ref()
                .and_then(|t| t.goodbytes.get()),
            Some(4.0)
        );

        let rows: Vec<DpiRow> = serde_json::from_value(serde_json::json!([
            {"by_app": [{"cat": 1, "app": 4, "tx_bytes": 10, "tx_packets": 2}]},
            {"cat": "9", "app": "1", "rx_bytes": 5}
        ]))
        .unwrap();
        let flat = flatten_dpi(rows);
        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].category(), "1");
        assert_eq!(flat[0].tx_packets.get(), Some(2.0));
    }
}
