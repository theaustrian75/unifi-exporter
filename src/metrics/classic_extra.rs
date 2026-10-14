use prometheus::{GaugeVec, IntGaugeVec, Registry};

use crate::poll::Snapshot;

use super::common::{gauge, igauge, set, set_i};

pub struct ClassicExtraMetrics {
    dhcp_active: GaugeVec,
    dhcp_pool: GaugeVec,
    dhcp_util: GaugeVec,
    dhcp_free: GaugeVec,
    dhcp_available: GaugeVec,
    dhcp_lease_start: GaugeVec,
    dhcp_lease_end: GaugeVec,
    dhcp_lease_time: GaugeVec,
    dhcp_is_static: GaugeVec,
    port_forward_enabled: GaugeVec,
    port_forward_log: GaugeVec,
    ssl_active: GaugeVec,
    ssl_valid: GaugeVec,
    ssl_from: GaugeVec,
    ssl_to: GaugeVec,
    topo_vertices: GaugeVec,
    topo_edges: GaugeVec,
    topo_devices: GaugeVec,
    topo_clients: GaugeVec,
    topo_unknown: GaugeVec,
    topo_wired: GaugeVec,
    topo_wireless: GaugeVec,
    topo_by_band: GaugeVec,
    topo_experience: GaugeVec,
    topo_rate: GaugeVec,
    topo_full_duplex: GaugeVec,
    country_rx: GaugeVec,
    country_tx: GaugeVec,
    ups_present: IntGaugeVec,
    port_anomaly_count: GaugeVec,
    port_anomaly_last: GaugeVec,
    unas_present: IntGaugeVec,
    unas_cpu: GaugeVec,
    unas_cpu_temp: GaugeVec,
    unas_mem_total: GaugeVec,
    unas_mem_free: GaugeVec,
    unas_mem_avail: GaugeVec,
    unas_rx: GaugeVec,
    unas_tx: GaugeVec,
    unas_pool_cap: GaugeVec,
    unas_pool_use: GaugeVec,
    unas_raid_cur: GaugeVec,
    unas_raid_exp: GaugeVec,
    unas_raid_prog: GaugeVec,
    unas_disk_size: GaugeVec,
    unas_disk_temp: GaugeVec,
    unas_disk_health: GaugeVec,
    unas_disk_poh: GaugeVec,
    unas_disk_rpm: GaugeVec,
    unas_disk_bad: GaugeVec,
    unas_disk_uncorr: GaugeVec,
    unas_disk_rer: GaugeVec,
    unas_disk_smart: GaugeVec,
    unas_disk_read: GaugeVec,
    unas_disk_write: GaugeVec,
    unas_share_quota: GaugeVec,
    unas_share_usage: GaugeVec,
    unas_share_members: GaugeVec,
}

impl ClassicExtraMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let net = ["network", "network_id", "site_name", "source"];
        let lease = [
            "ip", "mac", "hostname", "network", "network_id", "client_name", "site_name", "source",
        ];
        let pf = ["site_name", "name", "proto", "fwd_ip", "fwd_port", "dst_port"];
        let ssl = ["site_name", "cert_type", "subject", "issuer", "status"];
        let site = ["site_name", "source"];
        let link = ["uplink_mac", "downlink_mac", "link_type", "site_name", "source"];
        let band = ["band", "site_name", "source"];
        let ctry = ["code", "name", "region", "sub_region", "site_name", "source"];
        let anom = ["site_name", "source", "device_mac", "port_idx", "anomaly_type"];
        let unas_dev = ["source", "name", "model"];
        let unas_con = ["source", "name"];
        let pool = ["source", "name", "pool_id", "pool_type", "status"];
        let raid = [
            "source", "name", "pool_id", "raid_group_id", "current_level", "config_level",
        ];
        let disk = [
            "source", "name", "slot_id", "pool_id", "disk_type", "state", "model", "serial",
        ];
        let share = [
            "source", "name", "share_id", "share_name", "share_type", "status",
        ];
        Ok(Self {
            dhcp_active: gauge(registry, "unifi_dhcp_active_leases", "Active DHCP leases", &net)?,
            dhcp_pool: gauge(registry, "unifi_dhcp_pool_size", "DHCP pool size", &net)?,
            dhcp_util: gauge(registry, "unifi_dhcp_utilization_percent", "DHCP pool used %", &net)?,
            dhcp_free: gauge(registry, "unifi_dhcp_free_percent", "DHCP pool free %", &net)?,
            dhcp_available: gauge(registry, "unifi_dhcp_available_ips", "DHCP pool free IPs", &net)?,
            dhcp_lease_start: gauge(registry, "unifi_dhcp_lease_start", "Lease start unix time", &lease)?,
            dhcp_lease_end: gauge(registry, "unifi_dhcp_lease_end", "Lease end unix time", &lease)?,
            dhcp_lease_time: gauge(registry, "unifi_dhcp_lease_time", "Lease duration seconds", &lease)?,
            dhcp_is_static: gauge(registry, "unifi_dhcp_is_static", "1 if static DHCP mapping", &lease)?,
            port_forward_enabled: gauge(registry, "unifi_port_forward_enabled", "Port forward enabled", &pf)?,
            port_forward_log: gauge(registry, "unifi_port_forward_log", "Port forward logging", &pf)?,
            ssl_active: gauge(registry, "unifi_ssl_cert_active", "SSL cert is active", &ssl)?,
            ssl_valid: gauge(registry, "unifi_ssl_cert_valid", "SSL cert is valid", &ssl)?,
            ssl_from: gauge(registry, "unifi_ssl_cert_valid_from_seconds", "SSL cert not-before", &ssl)?,
            ssl_to: gauge(registry, "unifi_ssl_cert_valid_to_seconds", "SSL cert expiry", &ssl)?,
            topo_vertices: gauge(registry, "unifi_topology_vertices_total", "Topology vertices", &site)?,
            topo_edges: gauge(registry, "unifi_topology_edges_total", "Topology edges", &site)?,
            topo_devices: gauge(registry, "unifi_topology_devices_total", "Topology devices", &site)?,
            topo_clients: gauge(registry, "unifi_topology_clients_total", "Topology clients", &site)?,
            topo_unknown: gauge(registry, "unifi_topology_has_unknown_switch", "Unknown switch in topology", &site)?,
            topo_wired: gauge(registry, "unifi_topology_connections_wired", "Wired topology links", &site)?,
            topo_wireless: gauge(registry, "unifi_topology_connections_wireless", "Wireless topology links", &site)?,
            topo_by_band: gauge(registry, "unifi_topology_connections_by_band", "Wireless links by band", &band)?,
            topo_experience: gauge(registry, "unifi_topology_link_experience_score", "Wireless link experience", &link)?,
            topo_rate: gauge(registry, "unifi_topology_link_rate_mbps", "Wired link rate", &link)?,
            topo_full_duplex: gauge(registry, "unifi_topology_wired_full_duplex", "Full-duplex wired links", &site)?,
            country_rx: gauge(registry, "unifi_countrytraffic_receive_bytes_total", "Country RX bytes", &ctry)?,
            country_tx: gauge(registry, "unifi_countrytraffic_transmit_bytes_total", "Country TX bytes", &ctry)?,
            ups_present: igauge(registry, "unifi_ups_device_present", "UPS selector present", &["site_name", "mac", "label"])?,
            port_anomaly_count: gauge(registry, "unifi_port_anomaly_count", "Port anomaly events", &anom)?,
            port_anomaly_last: gauge(registry, "unifi_port_anomaly_last_seen", "Last port anomaly unix time", &anom)?,
            unas_present: igauge(registry, "unifi_unas_device_present", "UNAS console present", &unas_dev)?,
            unas_cpu: gauge(registry, "unifi_unas_cpu_load_percent", "UNAS CPU load", &unas_con)?,
            unas_cpu_temp: gauge(registry, "unifi_unas_cpu_temperature_celsius", "UNAS CPU temperature", &unas_con)?,
            unas_mem_total: gauge(registry, "unifi_unas_memory_total_bytes", "UNAS memory total", &unas_con)?,
            unas_mem_free: gauge(registry, "unifi_unas_memory_free_bytes", "UNAS memory free", &unas_con)?,
            unas_mem_avail: gauge(registry, "unifi_unas_memory_available_bytes", "UNAS memory available", &unas_con)?,
            unas_rx: gauge(registry, "unifi_unas_receive_kbps", "UNAS RX KB/s", &unas_con)?,
            unas_tx: gauge(registry, "unifi_unas_transmit_kbps", "UNAS TX KB/s", &unas_con)?,
            unas_pool_cap: gauge(registry, "unifi_unas_pool_capacity_bytes", "UNAS pool capacity", &pool)?,
            unas_pool_use: gauge(registry, "unifi_unas_pool_usage_bytes", "UNAS pool usage", &pool)?,
            unas_raid_cur: gauge(registry, "unifi_unas_raid_group_current_protection", "UNAS RAID current protection", &raid)?,
            unas_raid_exp: gauge(registry, "unifi_unas_raid_group_expected_protection", "UNAS RAID expected protection", &raid)?,
            unas_raid_prog: gauge(registry, "unifi_unas_raid_group_progress_percent", "UNAS RAID rebuild progress", &raid)?,
            unas_disk_size: gauge(registry, "unifi_unas_disk_size_bytes", "UNAS disk size", &disk)?,
            unas_disk_temp: gauge(registry, "unifi_unas_disk_temperature_celsius", "UNAS disk temperature", &disk)?,
            unas_disk_health: gauge(registry, "unifi_unas_disk_health_score", "UNAS disk health", &disk)?,
            unas_disk_poh: gauge(registry, "unifi_unas_disk_power_on_hours", "UNAS disk power-on hours", &disk)?,
            unas_disk_rpm: gauge(registry, "unifi_unas_disk_rpm", "UNAS disk RPM", &disk)?,
            unas_disk_bad: gauge(registry, "unifi_unas_disk_bad_sectors", "UNAS disk bad sectors", &disk)?,
            unas_disk_uncorr: gauge(registry, "unifi_unas_disk_uncorrectable_sectors", "UNAS uncorrectable sectors", &disk)?,
            unas_disk_rer: gauge(registry, "unifi_unas_disk_read_error_rate", "UNAS disk read error rate", &disk)?,
            unas_disk_smart: gauge(registry, "unifi_unas_disk_smart_read_errors", "UNAS SMART read errors", &disk)?,
            unas_disk_read: gauge(registry, "unifi_unas_disk_read_kbps", "UNAS disk read KB/s", &disk)?,
            unas_disk_write: gauge(registry, "unifi_unas_disk_write_kbps", "UNAS disk write KB/s", &disk)?,
            unas_share_quota: gauge(registry, "unifi_unas_share_quota_bytes", "UNAS share quota", &share)?,
            unas_share_usage: gauge(registry, "unifi_unas_share_usage_bytes", "UNAS share usage", &share)?,
            unas_share_members: gauge(registry, "unifi_unas_share_members", "UNAS share members", &share)?,
        })
    }

    pub fn apply(&self, snap: &Snapshot) {
        self.reset();
        let source = snap.source.as_str();
        for site in &snap.sites {
            let site_name = site.site.name.as_deref().unwrap_or("");
            emit_dhcp(self, site, site_name, source);
            for pf in &site.port_forwards {
                let labels = [
                    site_name,
                    pf.name.as_deref().unwrap_or(""),
                    pf.proto.as_deref().unwrap_or(""),
                    pf.fwd_ip.as_deref().unwrap_or(""),
                    pf.fwd_port.as_str(),
                    pf.dst_port.as_str(),
                ];
                set(&self.port_forward_enabled, &labels, Some(if pf.enabled.truthy() { 1.0 } else { 0.0 }));
                set(&self.port_forward_log, &labels, Some(if pf.log.truthy() { 1.0 } else { 0.0 }));
            }
            for cert in &site.ssl_certs {
                let labels = [
                    site_name,
                    cert.cert_type.as_deref().unwrap_or(""),
                    cert.subject.as_deref().unwrap_or(""),
                    cert.issuer.as_deref().unwrap_or(""),
                    cert.status.as_deref().unwrap_or(""),
                ];
                set(&self.ssl_active, &labels, Some(if cert.is_active.truthy() { 1.0 } else { 0.0 }));
                set(&self.ssl_valid, &labels, Some(if cert.is_valid.truthy() { 1.0 } else { 0.0 }));
                set(&self.ssl_from, &labels, cert.valid_from.get());
                set(&self.ssl_to, &labels, cert.valid_to.get());
            }
            if let Some(t) = &site.topology {
                emit_topology(self, t, site_name, source);
            }
            for c in &site.country_traffic {
                let code = c.code();
                let labels = [code, code, "", "", site_name, source];
                set(&self.country_rx, &labels, c.rx());
                set(&self.country_tx, &labels, c.tx());
            }
            for u in &site.ups_devices {
                set_i(
                    &self.ups_present,
                    &[
                        site_name,
                        u.mac.as_deref().unwrap_or(""),
                        u.label.as_deref().or(u.name.as_deref()).unwrap_or(""),
                    ],
                    1,
                );
            }
            for a in &site.port_anomalies {
                let labels = [
                    site_name,
                    source,
                    a.device_mac.as_deref().unwrap_or(""),
                    a.port_idx.as_str(),
                    a.anomaly_type.as_deref().unwrap_or(""),
                ];
                set(&self.port_anomaly_count, &labels, a.count.get());
                set(&self.port_anomaly_last, &labels, a.last_seen.get());
            }
        }
        if let Some(unas) = &snap.unas {
            emit_unas(self, unas, source);
        }
    }

    fn reset(&self) {
        self.dhcp_active.reset();
        self.dhcp_pool.reset();
        self.dhcp_util.reset();
        self.dhcp_free.reset();
        self.dhcp_available.reset();
        self.dhcp_lease_start.reset();
        self.dhcp_lease_end.reset();
        self.dhcp_lease_time.reset();
        self.dhcp_is_static.reset();
        self.port_forward_enabled.reset();
        self.port_forward_log.reset();
        self.ssl_active.reset();
        self.ssl_valid.reset();
        self.ssl_from.reset();
        self.ssl_to.reset();
        self.topo_vertices.reset();
        self.topo_edges.reset();
        self.topo_devices.reset();
        self.topo_clients.reset();
        self.topo_unknown.reset();
        self.topo_wired.reset();
        self.topo_wireless.reset();
        self.topo_by_band.reset();
        self.topo_experience.reset();
        self.topo_rate.reset();
        self.topo_full_duplex.reset();
        self.country_rx.reset();
        self.country_tx.reset();
        self.ups_present.reset();
        self.port_anomaly_count.reset();
        self.port_anomaly_last.reset();
        self.unas_present.reset();
        self.unas_cpu.reset();
        self.unas_cpu_temp.reset();
        self.unas_mem_total.reset();
        self.unas_mem_free.reset();
        self.unas_mem_avail.reset();
        self.unas_rx.reset();
        self.unas_tx.reset();
        self.unas_pool_cap.reset();
        self.unas_pool_use.reset();
        self.unas_raid_cur.reset();
        self.unas_raid_exp.reset();
        self.unas_raid_prog.reset();
        self.unas_disk_size.reset();
        self.unas_disk_temp.reset();
        self.unas_disk_health.reset();
        self.unas_disk_poh.reset();
        self.unas_disk_rpm.reset();
        self.unas_disk_bad.reset();
        self.unas_disk_uncorr.reset();
        self.unas_disk_rer.reset();
        self.unas_disk_smart.reset();
        self.unas_disk_read.reset();
        self.unas_disk_write.reset();
        self.unas_share_quota.reset();
        self.unas_share_usage.reset();
        self.unas_share_members.reset();
    }
}

fn emit_dhcp(m: &ClassicExtraMetrics, site: &crate::poll::SiteSnapshot, site_name: &str, source: &str) {
    use std::collections::HashMap;
    let mut counts: HashMap<String, usize> = HashMap::new();
    for l in &site.dhcp_leases {
        let nid = l.network_id().to_string();
        *counts.entry(nid).or_insert(0) += 1;
        let labels = [
            l.ip.as_deref().unwrap_or(""),
            l.mac.as_deref().unwrap_or(""),
            l.hostname.as_deref().unwrap_or(""),
            l.network.as_deref().unwrap_or(""),
            l.network_id(),
            l.client_name.as_deref().unwrap_or(""),
            site_name,
            source,
        ];
        set(&m.dhcp_lease_start, &labels, l.start());
        set(&m.dhcp_lease_end, &labels, l.end());
        set(&m.dhcp_lease_time, &labels, l.lease_time.get());
        set(
            &m.dhcp_is_static,
            &labels,
            Some(if l.is_static.truthy() { 1.0 } else { 0.0 }),
        );
    }
    for net in &site.networkconf {
        let Some(pool) = net.pool_size() else {
            continue;
        };
        let id = net._id.as_deref().unwrap_or("");
        let active = *counts.get(id).unwrap_or(&0) as f64;
        let available = (pool - active).max(0.0);
        let util = if pool > 0.0 { active / pool * 100.0 } else { 0.0 };
        let labels = [
            net.name.as_deref().unwrap_or(""),
            id,
            site_name,
            source,
        ];
        set(&m.dhcp_pool, &labels, Some(pool));
        set(&m.dhcp_active, &labels, Some(active));
        set(&m.dhcp_util, &labels, Some(util));
        set(&m.dhcp_free, &labels, Some(100.0 - util));
        set(&m.dhcp_available, &labels, Some(available));
    }
}

fn emit_topology(
    m: &ClassicExtraMetrics,
    t: &crate::client::classic::extra::Topology,
    site_name: &str,
    source: &str,
) {
    use std::collections::HashMap;
    let site = [site_name, source];
    let mut devices = 0.0;
    let mut clients = 0.0;
    for v in &t.vertices {
        match v.vertex_type.as_deref() {
            Some("DEVICE") => devices += 1.0,
            Some("CLIENT") => clients += 1.0,
            _ => {}
        }
    }
    let mut wired = 0.0;
    let mut wireless = 0.0;
    let mut full_duplex = 0.0;
    let mut bands: HashMap<String, f64> = HashMap::new();
    for e in &t.edges {
        let link = [
            e.uplink_mac.as_deref().unwrap_or(""),
            e.downlink_mac.as_deref().unwrap_or(""),
            e.edge_type.as_deref().unwrap_or(""),
            site_name,
            source,
        ];
        match e.edge_type.as_deref() {
            Some("WIRED") => {
                wired += 1.0;
                if e.duplex.as_deref() == Some("FULL_DUPLEX") {
                    full_duplex += 1.0;
                }
                set(&m.topo_rate, &link, e.rate_mbps.get());
            }
            Some("WIRELESS") => {
                wireless += 1.0;
                if let Some(band) = &e.radio_band {
                    *bands.entry(band.clone()).or_insert(0.0) += 1.0;
                }
                set(&m.topo_experience, &link, e.experience_score.get());
            }
            _ => {}
        }
    }
    set(&m.topo_vertices, &site, Some(t.vertices.len() as f64));
    set(&m.topo_edges, &site, Some(t.edges.len() as f64));
    set(&m.topo_devices, &site, Some(devices));
    set(&m.topo_clients, &site, Some(clients));
    set(
        &m.topo_unknown,
        &site,
        Some(if t.has_unknown_switch.truthy() { 1.0 } else { 0.0 }),
    );
    set(&m.topo_wired, &site, Some(wired));
    set(&m.topo_wireless, &site, Some(wireless));
    set(&m.topo_full_duplex, &site, Some(full_duplex));
    for (band, n) in bands {
        set(&m.topo_by_band, &[band.as_str(), site_name, source], Some(n));
    }
}

fn emit_unas(m: &ClassicExtraMetrics, unas: &crate::client::classic::extra::UnasSnapshot, source: &str) {
    let name = unas.name();
    let model = unas.model();
    set_i(&m.unas_present, &[source, name, model], 1);
    let console = [source, name];
    if let Some(info) = &unas.info {
        if let Some(cpu) = &info.cpu {
            set(&m.unas_cpu, &console, cpu.current_load.get());
            set(&m.unas_cpu_temp, &console, cpu.temperature.get());
        }
        if let Some(mem) = &info.memory {
            set(&m.unas_mem_total, &console, mem.total.get());
            set(&m.unas_mem_free, &console, mem.free.get());
            set(&m.unas_mem_avail, &console, mem.available.get());
        }
    }
    if let Some(io) = &unas.network_io {
        set(&m.unas_rx, &console, io.receive_kbps.get());
        set(&m.unas_tx, &console, io.transmit_kbps.get());
    }
    if let Some(storage) = &unas.storage {
        for p in &storage.pools {
            let pid = p.id.as_deref().unwrap_or("");
            let pl = [
                source,
                name,
                pid,
                p.pool_type.as_deref().unwrap_or(""),
                p.status.as_deref().unwrap_or(""),
            ];
            set(&m.unas_pool_cap, &pl, p.capacity.get());
            set(&m.unas_pool_use, &pl, p.usage.get());
            for rg in &p.raid_groups {
                let rl = [
                    source,
                    name,
                    pid,
                    rg.id.as_deref().unwrap_or(""),
                    rg.current_level.as_deref().unwrap_or(""),
                    rg.config_level.as_deref().unwrap_or(""),
                ];
                set(&m.unas_raid_cur, &rl, rg.current_protection.get());
                set(&m.unas_raid_exp, &rl, rg.expected_protection.get());
                set(&m.unas_raid_prog, &rl, rg.progress.get());
            }
        }
        for d in &storage.disks {
            let dl = [
                source,
                name,
                d.slot_id.as_deref().unwrap_or(""),
                d.pool_id.as_deref().unwrap_or(""),
                d.disk_type.as_deref().unwrap_or(""),
                d.state.as_deref().unwrap_or(""),
                d.model.as_deref().unwrap_or(""),
                d.serial.as_deref().unwrap_or(""),
            ];
            set(&m.unas_disk_size, &dl, d.size.get());
            set(&m.unas_disk_temp, &dl, d.temperature.get());
            set(&m.unas_disk_health, &dl, d.health_score.get());
            set(&m.unas_disk_poh, &dl, d.power_on_hours.get());
            set(&m.unas_disk_rpm, &dl, d.rpm.get());
            set(&m.unas_disk_bad, &dl, d.bad_sector_count.get());
            set(&m.unas_disk_uncorr, &dl, d.uncorrectable_sector_count.get());
            set(&m.unas_disk_rer, &dl, d.read_error_rate.get());
            set(&m.unas_disk_smart, &dl, d.smart_read_error_count.get());
            set(&m.unas_disk_read, &dl, d.read_kbps.get());
            set(&m.unas_disk_write, &dl, d.write_kbps.get());
        }
    }
    for s in &unas.drives {
        let sl = [
            source,
            name,
            s.id.as_deref().unwrap_or(""),
            s.name.as_deref().unwrap_or(""),
            s.drive_type.as_deref().unwrap_or(""),
            s.status.as_deref().unwrap_or(""),
        ];
        set(&m.unas_share_quota, &sl, s.quota.get());
        set(&m.unas_share_usage, &sl, s.usage.get());
        set(&m.unas_share_members, &sl, s.member_count.get());
    }
}
