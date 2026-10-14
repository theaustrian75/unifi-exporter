//! Serde models for Integration API GET responses.
//! Unknown fields are ignored so newer consoles stay compatible.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfo {
    pub application_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub data: Vec<T>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub count: Option<i64>,
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: String,
    pub name: Option<String>,
    pub internal_reference: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    pub name: Option<String>,
    pub model: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub state: Option<String>,
    pub firmware_version: Option<String>,
    pub firmware_updatable: Option<bool>,
    pub supported: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_features")]
    pub features: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_interfaces")]
    pub interfaces: DeviceInterfaces,
}

impl Device {
    /// Best-effort classic-style type from Integration `features`.
    /// Prefer a MAC join against classic `/stat/device` when that poll succeeded.
    pub fn inferred_type(&self) -> &'static str {
        let has = |name: &str| self.features.iter().any(|f| f.eq_ignore_ascii_case(name));
        if has("gateway") {
            "udm"
        } else if has("accessPoint") {
            "uap"
        } else if has("switching") {
            "usw"
        } else {
            ""
        }
    }
}

fn deserialize_interfaces<'de, D>(deserializer: D) -> Result<DeviceInterfaces, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(value).unwrap_or_default())
}

fn deserialize_features<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(match value {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        serde_json::Value::String(s) if !s.is_empty() => vec![s],
        _ => Vec::new(),
    })
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInterfaces {
    #[serde(default)]
    pub ports: Vec<PortOverview>,
    #[serde(default)]
    pub radios: Vec<RadioOverview>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortOverview {
    pub idx: Option<i64>,
    pub state: Option<String>,
    pub connector: Option<String>,
    pub speed_mbps: Option<i64>,
    pub max_speed_mbps: Option<i64>,
    pub poe: Option<PoeOverview>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PoeOverview {
    pub enabled: Option<bool>,
    pub standard: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub poe_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RadioOverview {
    pub frequency_g_hz: Option<f64>,
    pub channel: Option<i64>,
    pub channel_width_m_hz: Option<i64>,
    pub wlan_standard: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatistics {
    pub cpu_utilization_pct: Option<f64>,
    pub memory_utilization_pct: Option<f64>,
    #[serde(default, rename = "loadAverage1Min")]
    pub load_average1_min: Option<f64>,
    #[serde(default, rename = "loadAverage5Min")]
    pub load_average5_min: Option<f64>,
    #[serde(default, rename = "loadAverage15Min")]
    pub load_average15_min: Option<f64>,
    pub uptime_sec: Option<i64>,
    pub uplink: Option<UplinkStatistics>,
    #[serde(default)]
    pub uplinks: Vec<UplinkStatistics>,
    pub interfaces: Option<DeviceInterfaceStatistics>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UplinkStatistics {
    pub rx_rate_bps: Option<i64>,
    pub tx_rate_bps: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInterfaceStatistics {
    #[serde(default)]
    pub radios: Vec<RadioStatistics>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RadioStatistics {
    pub frequency_g_hz: Option<f64>,
    pub tx_retries_pct: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub client_type: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub connected_at: Option<String>,
    pub uplink_device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntity {
    pub id: String,
    pub name: Option<String>,
    pub enabled: Option<bool>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub network: Option<String>,
    pub action: Option<String>,
    pub index: Option<f64>,
    pub domain: Option<String>,
    pub metadata: Option<EntityMetadata>,
    pub security_configuration: Option<SecurityConfiguration>,
    #[serde(default)]
    pub members: Vec<serde_json::Value>,
    #[serde(default)]
    pub lags: Vec<serde_json::Value>,
    #[serde(default)]
    pub peers: Vec<serde_json::Value>,
    #[serde(default)]
    pub network_ids: Vec<String>,
}

impl NamedEntity {
    pub fn origin(&self) -> &str {
        self.metadata
            .as_ref()
            .and_then(|m| m.origin.as_deref())
            .unwrap_or("")
    }

    pub fn security_type(&self) -> &str {
        self.security_configuration
            .as_ref()
            .and_then(|s| s.security_type.as_deref())
            .unwrap_or("")
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityMetadata {
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityConfiguration {
    #[serde(rename = "type")]
    pub security_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PendingDevice {
    pub id: Option<String>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub mac_address: Option<String>,
    pub state: Option<String>,
    pub firmware_version: Option<String>,
    pub firmware_updatable: Option<bool>,
    pub supported: Option<bool>,
}

/// Generic inventory row used when we only need id/name (and optional type/state).
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub state: Option<String>,
    pub enabled: Option<bool>,
    pub action: Option<String>,
    pub index: Option<f64>,
    pub domain: Option<String>,
    pub code: Option<String>,
    pub authorized_guest_count: Option<f64>,
    pub authorized_guest_limit: Option<f64>,
    pub data_usage_limit_m_bytes: Option<f64>,
    pub time_limit_minutes: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DpiCatalogEntry {
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub id: Option<String>,
    pub name: Option<String>,
}

fn deserialize_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    Ok(match v {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) if s.is_empty() => None,
        serde_json::Value::String(s) => Some(s),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_type_from_features() {
        let d: Device = serde_json::from_value(serde_json::json!({
            "id": "abc",
            "name": "Office AP",
            "model": "U6-LR",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "features": ["accessPoint"]
        }))
        .unwrap();
        assert_eq!(d.inferred_type(), "uap");
        assert_eq!(d.name.as_deref(), Some("Office AP"));
    }
}
