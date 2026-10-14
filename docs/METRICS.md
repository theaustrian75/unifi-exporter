# Metrics reference

Hybrid read-only exporter. Series names follow [unpoller](https://github.com/unpoller/unpoller) where the data exists.

Sources:

- **Integration** — `GET /proxy/network/integration/v1/...` with `X-API-KEY`
- **Classic** — UniFi OS login then `GET /proxy/network/api/s/{site}/stat/...`

The only non-GET is classic `POST /api/auth/login` (session). Speedtest is **not** triggered.

## Exporter health

| Metric | Source |
|---|---|
| `unifi_exporter_up` | process |
| `unifi_exporter_scrape_success` | last poll (either source ok) |
| `unifi_exporter_integration_up` | Integration poll |
| `unifi_exporter_classic_up` | classic poll |
| `unifi_exporter_cache_age_seconds` | cache |
| `unifi_exporter_poll_duration_seconds` | last poll |
| `unifi_exporter_poll_errors_total` | both sources failed |

## Clients (classic `/stat/sta`)

Unpoller-compatible: `unifi_client_receive_bytes_total`, `unifi_client_transmit_bytes_total`, `unifi_client_rssi_db`, `unifi_client_radio_signal_db`, `unifi_client_noise_db`, `unifi_client_ccq_ratio`, `unifi_client_satisfaction_ratio`, MCS/NSS/PHY rates, retries, roam, uptime, wired counterparts.

Labels (unpoller-style): `name`, `mac`, `site_name`, `gw_name`, `sw_name`, `vlan`, `ip`, `oui`, `network`, `sw_port`, `ap_name`, `source`, `identity`, `wired` (+ radio labels on Wi-Fi series).

If classic is disabled, only `unifi_clients` (count from Integration presence) is exported.

## Devices (classic `/stat/device`, Integration fallback)

`unifi_device_info`, `unifi_device_present`, `unifi_device_up`, `unifi_device_uptime_seconds`, `unifi_device_cpu_utilization_ratio`, `unifi_device_memory_utilization_ratio`, `unifi_device_load_average_{1,5,15}`, memory byte gauges, tx/rx byte totals, temperatures, stations, upgradable, uplink latency/speed/uptime, `unifi_device_fan_level`, `unifi_device_storage`, `unifi_device_max_power_total`.

WAN from gateway `stat.gw`: `unifi_device_wan_receive_bytes_total`, `unifi_device_wan_transmit_bytes_total`, packets/drops/errors/rates, plus broadcast/multicast and LAN counterparts.

Switch totals from `stat.sw`: `unifi_device_switch_*`. AP user/guest counters from `stat.ap`: `unifi_device_stat_*`.

Labels: `type`, `site_name`, `name`, `source` (plus identity labels on `info`).

### Ports (`port_table`, Integration port overview fallback)

Unpoller-compatible `unifi_device_port_*` (bytes/packets/errors/drops/broadcast/multicast/rates, satisfaction, speed, PoE, SFP) plus `unifi_device_port_up`.

### Radios and VAPs (`radio_table` / `radio_table_stats` / `vap_table`)

`unifi_device_radio_*` (channel, utilization, TX power/retries/packets, stations) and `unifi_device_vap_*` (bytes/packets/errors/retries/satisfaction/signal).

Integration-only fallback: port speed/up from device interfaces, radio channel, and `unifi_device_radio_transmit_retries_ratio` from `statistics/latest` (`txRetriesPct`).

## Site health (classic `/stat/health`)

`unifi_site_health_status`, `unifi_site_num_user`, `unifi_site_num_guest`, `unifi_site_stations`, `unifi_site_uptime_seconds`, `unifi_site_latency_seconds`, site RX/TX rates, plus adopted/AP/switch/gateway/IoT/disconnected/pending counts, WAN drops, and health speedtest fields.

## Rogue APs / DPI / speedtest status / sysinfo (classic GET)

- Rogue: `unifi_rogue_ap_rssi_db`, age, noise, signal, bw, channel, frequency
- Site DPI (`/stat/dpi`): `unifi_dpi_{transmit,receive}_{bytes,packets}`
- Client DPI (`/stat/stadpi`): `unifi_client_dpi_{transmit,receive}_{bytes,packets}`
- `unifi_device_speedtest_{download,upload,latency_seconds,rundate_seconds,runtime_seconds}` — last stored result only
- Controller (`/stat/sysinfo`): `unifi_controller_info`, `unifi_controller_uptime_seconds`, `unifi_controller_update_available`

## Inventory (Integration)

`unifi_inventory` / `unifi_inventory_info`, plus unpoller Integration families: `unifi_wifi_broadcast_enabled` (`site_name,name,network,security_type`), `unifi_vpn_server_enabled`, `unifi_radius_profile_present`, `unifi_traffic_matching_list_present`, `unifi_site_to_site_tunnel_present`, `unifi_acl_rule_enabled` / `unifi_acl_rule_index`, `unifi_dns_policy_enabled`, `unifi_firewall_zone_network_count`, `unifi_lag_member_count`, `unifi_mclag_domain_*`, `unifi_switch_stack_member_count`, `unifi_hotspot_voucher_*`, `unifi_integration_device_*` (`device_id,name,type,model,site_name`), `unifi_country_present`, `unifi_dpi_{application,category}_present`.

`unifi_application_info`, `unifi_sites`, `unifi_site_info`, `unifi_pending_devices`.

## PDU / UPS / UCI / UNAS

- PDU outlets (`outlet_table`): `unifi_device_outlet_*`
- UPS battery (`vbms_table`): `unifi_device_ups_*`
- UPS selector (`/stat/ups-devices`): `unifi_ups_device_present`
- UCI/DOCSIS (`ci_state_table`): `unifi_device_ci_state_operational`, `unifi_device_ci_state_info`
- UNAS Drive API: `unifi_unas_*` (CPU/memory, pools, RAID, disks, shares)

## DHCP, forwards, certs, topology, country traffic

Classic GETs (404s become empty):

- DHCP leases + `rest/networkconf`: `unifi_dhcp_*`
- Port forwards: `unifi_port_forward_enabled`, `unifi_port_forward_log`
- SSL (`/stat/active`): `unifi_ssl_cert_*`
- Topology: `unifi_topology_*`
- Country traffic (last hour): `unifi_countrytraffic_{receive,transmit}_bytes_total`
- Port anomalies: `unifi_port_anomaly_*`
- VPN remote-user health (`/stat/health` vpn subsystem): `unifi_site_remote_user_*`, `unifi_site_site_to_site_enabled`
- VAP TCP/latency internals: `unifi_device_vap_transmit_latency_*`, `unifi_device_vap_receive_latency_*`, `unifi_device_vap_transmit_goodbyes`

## Remaining gaps vs unpoller

Core client/device/port/radio/WAN/site series used by community dashboards match when unpoller’s Prometheus namespace is `unifi`. Default unpoller installs may use the `unpoller_` prefix instead.

### Name / label mismatches (same data, not drop-in)

| This exporter | Unpoller (namespace `unifi`) |
|---|---|
| `unifi_rogue_ap_rssi_db` (and `unifi_rogue_ap_*`) | `unifi_rogueap_rssi` (`unifi_rogueap_*`) |
| `unifi_dpi_*` | `unifi_site_dpi_*` |
| `unifi_site_num_user` / `unifi_site_num_guest` | `unifi_site_users` / `unifi_site_guests` |
| Device/port/radio/WAN labels omit `tag`; WAN omits `port` | Extra `tag` / `port` labels |
| Site health: `{site_name,subsystem,source}` | `{subsystem,status,site_name,source}` |
| Totals registered as gauges | Many totals registered as counters |
| DPI `category`/`application` are numeric IDs | Mapped names via unpoller’s DPI catalog |
| Exporter health: `unifi_exporter_*` | `unifi_controller_up`, `unifi_prometheus_*` |

### Not collected

VPN mesh (Site Magic) live tunnel health beyond `/stat/health` remote-user fields.

### Intentionally out of scope

| Gap | Why |
|---|---|
| Trigger a speedtest | Would require `/cmd/devmgr` |
| Live WebSocket events | Not polled |
| Protect cameras | Different product API |
| Historical `stat/report/*` windows | Not in v1 poll |
| InfluxDB / Loki outputs | Prometheus only |

Classic wins overlapping telemetry; Integration fills inventory the classic API lacks.
