# unifi-exporter

Read-only Prometheus exporter for Ubiquiti UniFi Dream Machines. Hybrid collector:

- **Official Integration API** (`X-API-KEY`) for inventory and modern list endpoints
- **Classic UniFi OS controller API** (local username/password, cookie session) for client RF/traffic, WAN counters, and rich device stats

Metric names match [unpoller](https://github.com/unpoller/unpoller) where data exists (`unifi_client_rssi_db`, `unifi_device_cpu_utilization_ratio`, …).

**No write actions.** After session login, only allowlisted GETs are issued. Speedtest is status-read only (never triggered).

## Quick start (local hybrid)

1. Create an Integration API key: **Settings → Integrations → Create API Key** (view-only admin).
2. Create a **local** UniFi OS user (not SSO) with view-only rights for the classic API.
3. Copy [`config.example.toml`](config.example.toml) to `config.toml` and set `base_url`, `api_key`, `username`, and `password`.
4. Run (loads `./config.toml` automatically):

```bash
cargo run --release
```

Either source can be omitted; at least one is required. Full unpoller-like coverage needs both.

Metrics: `http://127.0.0.1:9130/metrics`  
Health: `http://127.0.0.1:9130/healthz`  
Ready (after first successful poll): `http://127.0.0.1:9130/readyz`  
Landing page: `http://127.0.0.1:9130/`

## Cloud Integration + local classic

Site Manager keys only reach the Integration API. Classic polls still need `base_url` plus local user/password on the LAN.

```bash
export UNIFI_EXPORTER_MODE=cloud
export UNIFI_EXPORTER_CONSOLE_ID=your-console-id
export UNIFI_EXPORTER_API_KEY=your-site-manager-api-key
export UNIFI_EXPORTER_BASE_URL=https://192.168.1.1
export UNIFI_EXPORTER_USERNAME=viewer
export UNIFI_EXPORTER_PASSWORD=...
cargo run --release
```

## Prometheus scrape config

```yaml
scrape_configs:
  - job_name: unifi
    scrape_interval: 30s
    static_configs:
      - targets: ["unifi-exporter:9130"]
```

Background polling defaults to **60s** (minimum 15s). Scrapes always read the in-memory cache.

## Grafana

Import dashboards from [`grafana/dashboards/`](grafana/dashboards/). See [`grafana/README.md`](grafana/README.md). Series catalog: [`docs/METRICS.md`](docs/METRICS.md).

## Docker

```bash
docker build -t unifi-exporter .
docker run --rm -p 9130:9130 \
  -e UNIFI_EXPORTER_MODE=local \
  -e UNIFI_EXPORTER_BASE_URL=https://192.168.1.1 \
  -e UNIFI_EXPORTER_API_KEY=... \
  -e UNIFI_EXPORTER_USERNAME=viewer \
  -e UNIFI_EXPORTER_PASSWORD=... \
  -e UNIFI_EXPORTER_TLS_INSECURE=true \
  unifi-exporter
```

## Read-only guarantee

- Integration client: GET-only, path allowlist, no `/actions`
- Classic client: `POST /api/auth/login` only to create a session; afterwards GET allowlisted `/proxy/network/api/s/{site}/stat/*`, `rest/{portforward,networkconf}`, `v2` topology/leases/country-traffic/port-anomalies, and UniFi Drive `/proxy/drive/api/v2/*` (UNAS).
- `/cmd/*`, adopt/unadopt, restarts, and speedtest triggers are not implemented

## License

[MIT](LICENSE), the same license as [unpoller](https://github.com/unpoller/unpoller).
