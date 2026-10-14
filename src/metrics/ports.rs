use prometheus::{GaugeVec, IntGaugeVec, Registry};

use crate::client::classic::models::{ClassicDevice, Port};
use crate::poll::{DeviceWithStats, Snapshot};

use super::common::{gauge, igauge, ratio, set, set_i};

const PORT: &[&str] = &[
    "port_id", "port_num", "port_name", "port_mac", "port_ip", "site_name", "name", "source",
];
const SFP: &[&str] = &[
    "sfp_part",
    "sfp_vendor",
    "sfp_serial",
    "sfp_compliance",
    "port_id",
    "port_num",
    "port_name",
    "port_mac",
    "port_ip",
    "site_name",
    "name",
    "source",
];

pub struct PortMetrics {
    up: IntGaugeVec,
    poe_amperes: GaugeVec,
    poe_watts: GaugeVec,
    poe_volts: GaugeVec,
    receive_broadcast_total: GaugeVec,
    receive_bytes_total: GaugeVec,
    receive_rate_bytes: GaugeVec,
    receive_dropped_total: GaugeVec,
    receive_errors_total: GaugeVec,
    receive_multicast_total: GaugeVec,
    receive_packets_total: GaugeVec,
    satisfaction_ratio: GaugeVec,
    port_speed_bps: GaugeVec,
    transmit_broadcast_total: GaugeVec,
    transmit_bytes_total: GaugeVec,
    transmit_rate_bytes: GaugeVec,
    transmit_dropped_total: GaugeVec,
    transmit_errors_total: GaugeVec,
    transmit_multicast_total: GaugeVec,
    transmit_packets_total: GaugeVec,
    sfp_current: GaugeVec,
    sfp_rx_power: GaugeVec,
    sfp_temperature: GaugeVec,
    sfp_tx_power: GaugeVec,
    sfp_voltage: GaugeVec,
}

impl PortMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let ns = "unifi_device_port_";
        Ok(Self {
            up: igauge(registry, "unifi_device_port_up", "1 if the switch port is up", PORT)?,
            poe_amperes: gauge(registry, &format!("{ns}poe_amperes"), "POE current", PORT)?,
            poe_watts: gauge(registry, &format!("{ns}poe_watts"), "POE power", PORT)?,
            poe_volts: gauge(registry, &format!("{ns}poe_volts"), "POE voltage", PORT)?,
            receive_broadcast_total: gauge(registry, &format!("{ns}receive_broadcast_total"), "Port RX broadcast", PORT)?,
            receive_bytes_total: gauge(registry, &format!("{ns}receive_bytes_total"), "Port RX bytes", PORT)?,
            receive_rate_bytes: gauge(registry, &format!("{ns}receive_rate_bytes"), "Port RX rate", PORT)?,
            receive_dropped_total: gauge(registry, &format!("{ns}receive_dropped_total"), "Port RX dropped", PORT)?,
            receive_errors_total: gauge(registry, &format!("{ns}receive_errors_total"), "Port RX errors", PORT)?,
            receive_multicast_total: gauge(registry, &format!("{ns}receive_multicast_total"), "Port RX multicast", PORT)?,
            receive_packets_total: gauge(registry, &format!("{ns}receive_packets_total"), "Port RX packets", PORT)?,
            satisfaction_ratio: gauge(registry, &format!("{ns}satisfaction_ratio"), "Port satisfaction", PORT)?,
            port_speed_bps: gauge(registry, &format!("{ns}port_speed_bps"), "Port speed", PORT)?,
            transmit_broadcast_total: gauge(registry, &format!("{ns}transmit_broadcast_total"), "Port TX broadcast", PORT)?,
            transmit_bytes_total: gauge(registry, &format!("{ns}transmit_bytes_total"), "Port TX bytes", PORT)?,
            transmit_rate_bytes: gauge(registry, &format!("{ns}transmit_rate_bytes"), "Port TX rate", PORT)?,
            transmit_dropped_total: gauge(registry, &format!("{ns}transmit_dropped_total"), "Port TX dropped", PORT)?,
            transmit_errors_total: gauge(registry, &format!("{ns}transmit_errors_total"), "Port TX errors", PORT)?,
            transmit_multicast_total: gauge(registry, &format!("{ns}transmit_multicast_total"), "Port TX multicast", PORT)?,
            transmit_packets_total: gauge(registry, &format!("{ns}transmit_packets_total"), "Port TX packets", PORT)?,
            sfp_current: gauge(registry, &format!("{ns}sfp_current"), "SFP current", SFP)?,
            sfp_rx_power: gauge(registry, &format!("{ns}sfp_rx_power"), "SFP RX power", SFP)?,
            sfp_temperature: gauge(registry, &format!("{ns}sfp_temperature"), "SFP temperature", SFP)?,
            sfp_tx_power: gauge(registry, &format!("{ns}sfp_tx_power"), "SFP TX power", SFP)?,
            sfp_voltage: gauge(registry, &format!("{ns}sfp_voltage"), "SFP voltage", SFP)?,
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
        self.up.reset();
        self.poe_amperes.reset();
        self.poe_watts.reset();
        self.poe_volts.reset();
        self.receive_broadcast_total.reset();
        self.receive_bytes_total.reset();
        self.receive_rate_bytes.reset();
        self.receive_dropped_total.reset();
        self.receive_errors_total.reset();
        self.receive_multicast_total.reset();
        self.receive_packets_total.reset();
        self.satisfaction_ratio.reset();
        self.port_speed_bps.reset();
        self.transmit_broadcast_total.reset();
        self.transmit_bytes_total.reset();
        self.transmit_rate_bytes.reset();
        self.transmit_dropped_total.reset();
        self.transmit_errors_total.reset();
        self.transmit_multicast_total.reset();
        self.transmit_packets_total.reset();
        self.sfp_current.reset();
        self.sfp_rx_power.reset();
        self.sfp_temperature.reset();
        self.sfp_tx_power.reset();
        self.sfp_voltage.reset();
    }
}

fn emit_classic(m: &PortMetrics, d: &ClassicDevice, site_name: &str, source: &str) {
    let name = d.display_name();
    for p in &d.port_table {
        if !p.alive() {
            continue;
        }
        emit_port(m, p, site_name, name.as_str(), source);
    }
}

fn emit_port(m: &PortMetrics, p: &Port, site_name: &str, name: &str, source: &str) {
    let idx = p.idx_label();
    let port_name = p.name.as_deref().unwrap_or("");
    let port_id = if port_name.is_empty() {
        format!("{name} Port {idx}")
    } else {
        format!("{name} {port_name}")
    };
    let labels = [
        port_id.as_str(),
        idx.as_str(),
        port_name,
        p.mac.as_deref().unwrap_or(""),
        p.ip.as_deref().unwrap_or(""),
        site_name,
        name,
        source,
    ];
    set_i(&m.up, &labels, if p.up.truthy() || p.up.get().is_none() { 1 } else { 0 });
    if p.poe_enable.truthy() || p.port_poe.truthy() || p.poe_power.get().unwrap_or(0.0) > 0.0 {
        set(&m.poe_amperes, &labels, p.poe_current.get());
        set(&m.poe_watts, &labels, p.poe_power.get());
        set(&m.poe_volts, &labels, p.poe_voltage.get());
    }
    set(&m.receive_broadcast_total, &labels, p.rx_broadcast.get());
    set(&m.receive_bytes_total, &labels, p.rx_bytes.get());
    set(&m.receive_rate_bytes, &labels, p.rx_bytes_r.get());
    set(&m.receive_dropped_total, &labels, p.rx_dropped.get());
    set(&m.receive_errors_total, &labels, p.rx_errors.get());
    set(&m.receive_multicast_total, &labels, p.rx_multicast.get());
    set(&m.receive_packets_total, &labels, p.rx_packets.get());
    set(&m.satisfaction_ratio, &labels, ratio(p.satisfaction.get()));
    set(&m.port_speed_bps, &labels, p.speed.get().map(|v| v * 1_000_000.0));
    set(&m.transmit_broadcast_total, &labels, p.tx_broadcast.get());
    set(&m.transmit_bytes_total, &labels, p.tx_bytes.get());
    set(&m.transmit_rate_bytes, &labels, p.tx_bytes_r.get());
    set(&m.transmit_dropped_total, &labels, p.tx_dropped.get());
    set(&m.transmit_errors_total, &labels, p.tx_errors.get());
    set(&m.transmit_multicast_total, &labels, p.tx_multicast.get());
    set(&m.transmit_packets_total, &labels, p.tx_packets.get());

    if p.sfp_found.truthy() {
        let sfp = [
            p.sfp_part.as_deref().unwrap_or(""),
            p.sfp_vendor.as_deref().unwrap_or(""),
            p.sfp_serial.as_deref().unwrap_or(""),
            p.sfp_compliance.as_deref().unwrap_or(""),
            labels[0],
            labels[1],
            labels[2],
            labels[3],
            labels[4],
            labels[5],
            labels[6],
            labels[7],
        ];
        set(&m.sfp_current, &sfp, p.sfp_current.get());
        set(&m.sfp_voltage, &sfp, p.sfp_voltage.get());
        set(&m.sfp_temperature, &sfp, p.sfp_temperature.get());
        set(&m.sfp_rx_power, &sfp, p.sfp_rxpower.get());
        set(&m.sfp_tx_power, &sfp, p.sfp_txpower.get());
    }
}

fn emit_integration(m: &PortMetrics, dws: &DeviceWithStats, site_name: &str, source: &str) {
    let d = &dws.device;
    let name = d.name.clone().unwrap_or_default();
    for p in &d.interfaces.ports {
        let idx = p.idx.map(|v| v.to_string()).unwrap_or_default();
        let port_name = p.connector.as_deref().unwrap_or("");
        let port_id = format!("{name} Port {idx}");
        let labels = [
            port_id.as_str(),
            idx.as_str(),
            port_name,
            "",
            "",
            site_name,
            name.as_str(),
            source,
        ];
        let up = p
            .state
            .as_deref()
            .map(|s| {
                let s = s.to_ascii_lowercase();
                s == "up" || s == "link_up" || s == "online" || s.contains("up")
            })
            .unwrap_or(false);
        set_i(&m.up, &labels, i64::from(up));
        set(
            &m.port_speed_bps,
            &labels,
            p.speed_mbps.map(|v| v as f64 * 1_000_000.0),
        );
    }
}
