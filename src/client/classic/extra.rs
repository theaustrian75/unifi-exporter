//! Extra classic JSON models: DHCP, port forwards, SSL, topology, country traffic, UPS, PDU, UCI, UNAS.

use serde::Deserialize;
use serde_json::Value;

use super::models::{value_f64, Flex, FlexStr};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpsSelector {
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DhcpLease {
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default, rename = "network_id")]
    pub network_id: Option<String>,
    #[serde(default, rename = "client_name")]
    pub client_name: Option<String>,
    #[serde(default, rename = "lease_start")]
    pub lease_start: Flex,
    #[serde(default, rename = "lease_end")]
    pub lease_end: Flex,
    #[serde(default, rename = "lease_time")]
    pub lease_time: Flex,
    #[serde(default, rename = "is_static")]
    pub is_static: Flex,
    #[serde(default)]
    pub start: Flex,
    #[serde(default)]
    pub end: Flex,
}

impl DhcpLease {
    pub fn start(&self) -> Option<f64> {
        self.lease_start.get().or_else(|| self.start.get())
    }

    pub fn end(&self) -> Option<f64> {
        self.lease_end.get().or_else(|| self.end.get())
    }

    pub fn network_id(&self) -> &str {
        self.network_id.as_deref().unwrap_or("")
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NetworkConf {
    #[serde(default)]
    pub _id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "dhcpd_start")]
    pub dhcpd_start: Option<String>,
    #[serde(default, rename = "dhcpd_stop")]
    pub dhcpd_stop: Option<String>,
    #[serde(default, rename = "dhcpd_enabled")]
    pub dhcpd_enabled: Flex,
}

impl NetworkConf {
    pub fn pool_size(&self) -> Option<f64> {
        let start = ipv4_u32(self.dhcpd_start.as_deref()?)?;
        let stop = ipv4_u32(self.dhcpd_stop.as_deref()?)?;
        if stop >= start {
            Some((stop - start + 1) as f64)
        } else {
            None
        }
    }
}

fn ipv4_u32(s: &str) -> Option<u32> {
    let mut out = 0u32;
    let mut parts = s.split('.');
    for _ in 0..4 {
        let p: u32 = parts.next()?.parse().ok()?;
        if p > 255 {
            return None;
        }
        out = (out << 8) | p;
    }
    if parts.next().is_some() {
        return None;
    }
    Some(out)
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PortForward {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub proto: Option<String>,
    #[serde(default, rename = "fwd")]
    pub fwd_ip: Option<String>,
    #[serde(default, rename = "fwd_port")]
    pub fwd_port: FlexStr,
    #[serde(default, rename = "dst_port")]
    pub dst_port: FlexStr,
    #[serde(default)]
    pub enabled: Flex,
    #[serde(default)]
    pub log: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SslCertificate {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "_id")]
    pub _id: Option<String>,
    #[serde(default, rename = "cert_type")]
    pub cert_type: Option<String>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "is_active")]
    pub is_active: Flex,
    #[serde(default, rename = "is_valid")]
    pub is_valid: Flex,
    #[serde(default, rename = "valid_from")]
    pub valid_from: Flex,
    #[serde(default, rename = "valid_to")]
    pub valid_to: Flex,
}

impl SslCertificate {
    pub fn id(&self) -> &str {
        self.id.as_deref().or(self._id.as_deref()).unwrap_or("")
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Topology {
    #[serde(default)]
    pub vertices: Vec<TopoVertex>,
    #[serde(default)]
    pub edges: Vec<TopoEdge>,
    #[serde(default, rename = "hasUnknownSwitch")]
    pub has_unknown_switch: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TopoVertex {
    #[serde(default, rename = "type")]
    pub vertex_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TopoEdge {
    #[serde(default, rename = "type")]
    pub edge_type: Option<String>,
    #[serde(default, rename = "uplinkMac")]
    pub uplink_mac: Option<String>,
    #[serde(default, rename = "downlinkMac")]
    pub downlink_mac: Option<String>,
    #[serde(default)]
    pub duplex: Option<String>,
    #[serde(default, rename = "radioBand")]
    pub radio_band: Option<String>,
    #[serde(default, rename = "rateMbps")]
    pub rate_mbps: Flex,
    #[serde(default, rename = "experienceScore")]
    pub experience_score: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CountryTraffic {
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default, rename = "bytes_received")]
    pub bytes_received: Flex,
    #[serde(default, rename = "bytes_transmitted")]
    pub bytes_transmitted: Flex,
    #[serde(default, rename = "rx_bytes")]
    pub rx_bytes: Flex,
    #[serde(default, rename = "tx_bytes")]
    pub tx_bytes: Flex,
}

impl CountryTraffic {
    pub fn code(&self) -> &str {
        self.country
            .as_deref()
            .or(self.code.as_deref())
            .unwrap_or("")
    }

    pub fn rx(&self) -> Option<f64> {
        self.bytes_received.get().or_else(|| self.rx_bytes.get())
    }

    pub fn tx(&self) -> Option<f64> {
        self.bytes_transmitted
            .get()
            .or_else(|| self.tx_bytes.get())
    }

    pub fn from_value(v: &Value) -> Option<Self> {
        Some(Self {
            country: v
                .get("country")
                .or_else(|| v.get("code"))
                .and_then(|x| x.as_str())
                .map(str::to_string),
            code: None,
            bytes_received: Flex(
                v.get("bytes_received")
                    .or_else(|| v.get("rx_bytes"))
                    .and_then(value_f64),
            ),
            bytes_transmitted: Flex(
                v.get("bytes_transmitted")
                    .or_else(|| v.get("tx_bytes"))
                    .and_then(value_f64),
            ),
            rx_bytes: Flex(None),
            tx_bytes: Flex(None),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PortAnomaly {
    #[serde(default, rename = "deviceMac", alias = "device_mac")]
    pub device_mac: Option<String>,
    #[serde(default, rename = "portIdx", alias = "port_idx")]
    pub port_idx: FlexStr,
    #[serde(default, rename = "anomalyType", alias = "anomaly_type")]
    pub anomaly_type: Option<String>,
    #[serde(default)]
    pub count: Flex,
    #[serde(default, rename = "lastSeen", alias = "last_seen")]
    pub last_seen: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasDeviceInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub cpu: Option<UnasCpu>,
    #[serde(default)]
    pub memory: Option<UnasMemory>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasCpu {
    #[serde(default, rename = "currentLoad", alias = "currentload")]
    pub current_load: Flex,
    #[serde(default)]
    pub temperature: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasMemory {
    #[serde(default)]
    pub total: Flex,
    #[serde(default)]
    pub free: Flex,
    #[serde(default)]
    pub available: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasStorage {
    #[serde(default)]
    pub pools: Vec<UnasPool>,
    #[serde(default)]
    pub disks: Vec<UnasDisk>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasPool {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "type")]
    pub pool_type: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub capacity: Flex,
    #[serde(default)]
    pub usage: Flex,
    #[serde(default, rename = "raidGroups")]
    pub raid_groups: Vec<UnasRaidGroup>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasRaidGroup {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "currentLevel")]
    pub current_level: Option<String>,
    #[serde(default, rename = "configLevel")]
    pub config_level: Option<String>,
    #[serde(default, rename = "currentProtection")]
    pub current_protection: Flex,
    #[serde(default, rename = "expectedProtection")]
    pub expected_protection: Flex,
    #[serde(default)]
    pub progress: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasDisk {
    #[serde(default, rename = "slotId")]
    pub slot_id: Option<String>,
    #[serde(default, rename = "poolId")]
    pub pool_id: Option<String>,
    #[serde(default, rename = "type")]
    pub disk_type: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub size: Flex,
    #[serde(default)]
    pub temperature: Flex,
    #[serde(default, rename = "healthScore")]
    pub health_score: Flex,
    #[serde(default, rename = "powerOnHours")]
    pub power_on_hours: Flex,
    #[serde(default, rename = "rpm")]
    pub rpm: Flex,
    #[serde(default, rename = "badSectorCount")]
    pub bad_sector_count: Flex,
    #[serde(default, rename = "uncorrectableSectorCount")]
    pub uncorrectable_sector_count: Flex,
    #[serde(default, rename = "readErrorRate")]
    pub read_error_rate: Flex,
    #[serde(default, rename = "smartReadErrorCount")]
    pub smart_read_error_count: Flex,
    #[serde(default, rename = "readKBPS")]
    pub read_kbps: Flex,
    #[serde(default, rename = "writeKBPS")]
    pub write_kbps: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasNetworkIo {
    #[serde(default, rename = "receiveKBPS")]
    pub receive_kbps: Flex,
    #[serde(default, rename = "transmitKBPS")]
    pub transmit_kbps: Flex,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UnasDrive {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub drive_type: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub quota: Flex,
    #[serde(default)]
    pub usage: Flex,
    #[serde(default, rename = "memberCount")]
    pub member_count: Flex,
}

#[derive(Debug, Clone, Default)]
pub struct UnasSnapshot {
    pub info: Option<UnasDeviceInfo>,
    pub storage: Option<UnasStorage>,
    pub network_io: Option<UnasNetworkIo>,
    pub drives: Vec<UnasDrive>,
}

impl UnasSnapshot {
    pub fn is_empty(&self) -> bool {
        self.info.is_none()
            && self.storage.is_none()
            && self.network_io.is_none()
            && self.drives.is_empty()
    }

    pub fn name(&self) -> &str {
        self.info
            .as_ref()
            .and_then(|i| i.name.as_deref())
            .unwrap_or("")
    }

    pub fn model(&self) -> &str {
        self.info
            .as_ref()
            .and_then(|i| i.model.as_deref())
            .unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dhcp_pool_size() {
        let n = NetworkConf {
            dhcpd_start: Some("192.168.1.100".into()),
            dhcpd_stop: Some("192.168.1.200".into()),
            ..Default::default()
        };
        assert_eq!(n.pool_size(), Some(101.0));
    }
}
