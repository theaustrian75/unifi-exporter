# Grafana dashboards

JSON dashboards use **unpoller-compatible** metric names from this hybrid exporter.

Client RF/traffic and WAN panels require classic API credentials (`username` / `password`).

See [`../docs/METRICS.md`](../docs/METRICS.md).

## Import (UI)

1. Grafana → Dashboards → New → Import
2. Upload `overview.json`, `devices.json`, `clients.json`, or `network.json`
3. Select your Prometheus datasource
