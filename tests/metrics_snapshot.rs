use std::time::Instant;

use unifi_exporter::client::classic::models::{
    ClassicClient, ClassicDevice, ClientDpi, DpiEntry, Flex, HealthSubsystem, Port, Radio, RogueAp,
    Sysinfo, Vap,
};
use unifi_exporter::client::models::{ApplicationInfo, Device, DeviceStatistics, Site};
use unifi_exporter::metrics::Metrics;
use unifi_exporter::poll::{DeviceWithStats, SiteSnapshot, Snapshot};

#[test]
fn renders_unpoller_metric_names_from_classic_snapshot() {
    let metrics = Metrics::new().expect("metrics");
    let snap = Snapshot {
        info: ApplicationInfo {
            application_version: Some("10.4.57".into()),
        },
        pending_devices: vec![],
        sites: vec![SiteSnapshot {
            site: Site {
                id: "site-1".into(),
                name: Some("Home".into()),
                internal_reference: Some("default".into()),
            },
            devices: vec![DeviceWithStats {
                device: Device {
                    id: "dev-uuid".into(),
                    name: Some("UDM".into()),
                    model: Some("UDM-SE".into()),
                    mac_address: Some("AA:BB:CC:DD:EE:FF".into()),
                    features: vec!["gateway".into()],
                    ..Default::default()
                },
                stats: Some(DeviceStatistics {
                    cpu_utilization_pct: Some(12.0),
                    ..Default::default()
                }),
            }],
            classic_clients: vec![ClassicClient {
                mac: Some("11:22:33:44:55:66".into()),
                ip: Some("192.168.1.50".into()),
                name: Some("phone".into()),
                rssi: Flex(Some(-62.0)),
                rx_bytes: Flex(Some(1000.0)),
                tx_bytes: Flex(Some(2000.0)),
                is_wired: Flex(Some(0.0)),
                ..Default::default()
            }],
            classic_devices: vec![ClassicDevice {
                device_type: Some("udm".into()),
                name: Some("UDM".into()),
                mac: Some("aa:bb:cc:dd:ee:ff".into()),
                ip: Some("192.168.1.1".into()),
                model: Some("UDM-SE".into()),
                version: Some("4.0.0".into()),
                state: Flex(Some(1.0)),
                port_table: vec![Port {
                    port_idx: Flex(Some(1.0)),
                    name: Some("Port 1".into()),
                    up: Flex(Some(1.0)),
                    enable: Flex(Some(1.0)),
                    rx_bytes: Flex(Some(100.0)),
                    tx_bytes: Flex(Some(200.0)),
                    speed: Flex(Some(1000.0)),
                    ..Default::default()
                }],
                radio_table: vec![Radio {
                    name: Some("wifi0".into()),
                    radio: Some("ng".into()),
                    channel: Flex(Some(6.0)),
                    cu_total: Flex(Some(12.0)),
                    tx_retries: Flex(Some(3.0)),
                    ..Default::default()
                }],
                vap_table: vec![Vap {
                    name: Some("ath0".into()),
                    essid: Some("Home".into()),
                    radio: Some("ng".into()),
                    up: Flex(Some(1.0)),
                    rx_bytes: Flex(Some(9.0)),
                    tx_bytes: Flex(Some(11.0)),
                    tx_tcp_stats: Some(unifi_exporter::client::classic::models::TcpStats {
                        goodbytes: Flex(Some(4.0)),
                        lat_avg: Flex(Some(12.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                outlet_table: vec![unifi_exporter::client::classic::models::Outlet {
                    index: Flex(Some(1.0)),
                    name: Some("Camera".into()),
                    relay_state: Flex(Some(1.0)),
                    outlet_power: Flex(Some(8.5)),
                    ..Default::default()
                }],
                ci_state_table: Some(unifi_exporter::client::classic::models::CiStateTable {
                    ci_state: Some("Operational".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            health: vec![HealthSubsystem {
                subsystem: Some("vpn".into()),
                status: Some("ok".into()),
                num_sta: Flex(Some(4.0)),
                uptime: Flex(Some(123.0)),
                remote_user_num_active: Flex(Some(2.0)),
                ..Default::default()
            }],
            dpi: vec![DpiEntry {
                cat: unifi_exporter::client::classic::models::FlexStr(Some("1".into())),
                app: unifi_exporter::client::classic::models::FlexStr(Some("4".into())),
                tx_bytes: Flex(Some(50.0)),
                tx_packets: Flex(Some(3.0)),
                ..Default::default()
            }],
            client_dpi: vec![ClientDpi {
                mac: Some("11:22:33:44:55:66".into()),
                name: Some("phone".into()),
                entries: vec![DpiEntry {
                    cat: unifi_exporter::client::classic::models::FlexStr(Some("1".into())),
                    app: unifi_exporter::client::classic::models::FlexStr(Some("4".into())),
                    rx_bytes: Flex(Some(7.0)),
                    rx_packets: Flex(Some(2.0)),
                    ..Default::default()
                }],
            }],
            rogue_aps: vec![RogueAp {
                essid: Some("Other".into()),
                bssid: Some("00:11:22:33:44:55".into()),
                rssi: Flex(Some(-70.0)),
                age: Flex(Some(30.0)),
                noise: Flex(Some(-90.0)),
                channel: Flex(Some(11.0)),
                ..Default::default()
            }],
            sysinfo: Some(Sysinfo {
                hostname: Some("UDM".into()),
                version: Some("4.0.0".into()),
                uptime: Flex(Some(999.0)),
                ..Default::default()
            }),
            dhcp_leases: vec![unifi_exporter::client::classic::extra::DhcpLease {
                ip: Some("192.168.1.50".into()),
                mac: Some("11:22:33:44:55:66".into()),
                hostname: Some("phone".into()),
                network: Some("LAN".into()),
                network_id: Some("net1".into()),
                is_static: Flex(Some(0.0)),
                ..Default::default()
            }],
            networkconf: vec![unifi_exporter::client::classic::extra::NetworkConf {
                _id: Some("net1".into()),
                name: Some("LAN".into()),
                dhcpd_start: Some("192.168.1.100".into()),
                dhcpd_stop: Some("192.168.1.200".into()),
                ..Default::default()
            }],
            port_forwards: vec![unifi_exporter::client::classic::extra::PortForward {
                name: Some("ssh".into()),
                proto: Some("tcp".into()),
                fwd_ip: Some("192.168.1.10".into()),
                fwd_port: unifi_exporter::client::classic::models::FlexStr(Some("22".into())),
                dst_port: unifi_exporter::client::classic::models::FlexStr(Some("22".into())),
                enabled: Flex(Some(1.0)),
                ..Default::default()
            }],
            ..Default::default()
        }],
        fetched_at: Some(Instant::now()),
        last_error: None,
        last_poll_duration: Some(std::time::Duration::from_millis(250)),
        poll_ok: true,
        integration_ok: true,
        classic_ok: true,
        source: "unifi.example".into(),
        ..Default::default()
    };

    let body = metrics.render(&snap).expect("render");
    assert!(body.contains("unifi_exporter_up 1"));
    assert!(body.contains("unifi_exporter_classic_up 1"));
    assert!(body.contains("unifi_application_info"));
    assert!(body.contains("unifi_devices 1"));
    assert!(body.contains("unifi_clients 1"));
    assert!(body.contains("unifi_device_present"));
    assert!(body.contains("unifi_device_info"));
    assert!(body.contains("unifi_client_rssi_db"));
    assert!(body.contains("unifi_client_receive_bytes_total"));
    assert!(body.contains("unifi_client_transmit_bytes_total"));
    assert!(body.contains("unifi_device_up"));
    assert!(body.contains("unifi_device_port_receive_bytes_total"));
    assert!(body.contains("unifi_device_radio_channel"));
    assert!(body.contains("unifi_device_vap_receive_bytes_total"));
    assert!(body.contains("unifi_dpi_transmit_packets"));
    assert!(body.contains("unifi_client_dpi_receive_bytes"));
    assert!(body.contains("unifi_rogue_ap_age_seconds"));
    assert!(body.contains("unifi_site_stations"));
    assert!(body.contains("unifi_site_uptime_seconds"));
    assert!(body.contains("unifi_controller_uptime_seconds"));
    assert!(body.contains("unifi_device_outlet_outlet_power"));
    assert!(body.contains("unifi_device_ci_state_operational"));
    assert!(body.contains("unifi_device_vap_transmit_latency_average_seconds"));
    assert!(body.contains("unifi_site_remote_user_active"));
    assert!(body.contains("unifi_dhcp_active_leases"));
    assert!(body.contains("unifi_port_forward_enabled"));
    assert!(body.contains("unifi_integration_device_cpu_utilization_pct"));
    assert!(body.contains("device_id=\"dev-uuid\""));
    assert!(body.contains("name=\"UDM\""));
    assert!(body.contains("model=\"UDM-SE\""));
}
