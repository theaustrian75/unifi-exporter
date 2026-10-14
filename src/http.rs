use std::sync::Arc;

use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;

use crate::metrics::Metrics;
use crate::poll::{Snapshot, SnapshotCache};

#[derive(Clone)]
pub struct AppState {
    pub cache: SnapshotCache,
    pub metrics: Arc<Metrics>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/metrics", get(metrics_handler))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let snap = state.cache.get().await;
    Html(landing_html(&snap))
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok\n")
}

async fn readyz(State(state): State<AppState>) -> Response {
    let snap = state.cache.get().await;
    if snap.fetched_at.is_some() {
        (StatusCode::OK, "ready\n").into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "not ready\n").into_response()
    }
}

async fn metrics_handler(State(state): State<AppState>) -> Response {
    let snap = state.cache.get().await;
    match state.metrics.render(&snap) {
        Ok(body) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
            body,
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to render metrics: {err}\n"),
        )
            .into_response(),
    }
}

fn landing_html(snap: &Snapshot) -> String {
    let version = env!("CARGO_PKG_VERSION");
    let poll = if snap.poll_ok { ("ok", "up") } else { ("fail", "down") };
    let integration = if snap.integration_ok { ("ok", "up") } else { ("fail", "down") };
    let classic = if snap.classic_ok { ("ok", "up") } else { ("fail", "down") };
    let cache_age = match snap.fetched_at {
        Some(t) => format!("{:.0}s", t.elapsed().as_secs_f64()),
        None => "never".to_string(),
    };
    let poll_dur = snap
        .last_poll_duration
        .map(|d| format!("{:.2}s", d.as_secs_f64()))
        .unwrap_or_else(|| "—".into());
    let mut devices = 0usize;
    let mut clients = 0usize;
    for site in &snap.sites {
        devices += if site.classic_devices.is_empty() {
            site.devices.len()
        } else {
            site.classic_devices.len()
        };
        clients += if site.classic_clients.is_empty() {
            site.clients.len()
        } else {
            site.classic_clients.len()
        };
    }
    let sites: String = snap
        .sites
        .iter()
        .map(|s| {
            let name = s.site.name.as_deref().unwrap_or("(unnamed)");
            let id = s.site.internal_reference.as_deref().unwrap_or(s.site.id.as_str());
            format!("<li><strong>{}</strong> <span class=\"muted\">{}</span></li>", escape(name), escape(id))
        })
        .collect();
    let sites_block = if sites.is_empty() {
        "<p class=\"muted\">No sites in the last snapshot.</p>".to_string()
    } else {
        format!("<ul class=\"sites\">{sites}</ul>")
    };
    let error_block = snap
        .last_error
        .as_deref()
        .filter(|e| !e.is_empty())
        .map(|e| format!("<p class=\"error\">Last error: {}</p>", escape(e)))
        .unwrap_or_default();
    let ready = if snap.fetched_at.is_some() { ("ok", "ready") } else { ("fail", "not ready") };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>unifi-exporter</title>
  <style>
    :root {{
      color-scheme: light dark;
      --bg: #0f1419;
      --card: #1a222c;
      --text: #e7ecf3;
      --muted: #8b9bb0;
      --line: #2a3644;
      --ok: #3dd68c;
      --fail: #ff6b6b;
      --link: #6cb6ff;
    }}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      font: 16px/1.5 ui-sans-serif, system-ui, sans-serif;
      background: var(--bg);
      color: var(--text);
    }}
    main {{ max-width: 44rem; margin: 2.5rem auto; padding: 0 1.25rem 3rem; }}
    h1 {{ font-size: 1.6rem; font-weight: 650; margin: 0 0 .25rem; letter-spacing: -.02em; }}
    .sub {{ color: var(--muted); margin: 0 0 1.5rem; }}
    .card {{
      background: var(--card);
      border: 1px solid var(--line);
      border-radius: 12px;
      padding: 1rem 1.15rem;
      margin: 0 0 1rem;
    }}
    h2 {{ font-size: .78rem; text-transform: uppercase; letter-spacing: .08em; color: var(--muted); margin: 0 0 .7rem; }}
    a {{ color: var(--link); text-decoration: none; }}
    a:hover {{ text-decoration: underline; }}
    dl {{ display: grid; grid-template-columns: 9rem 1fr; gap: .35rem 1rem; margin: 0; }}
    dt {{ color: var(--muted); }}
    dd {{ margin: 0; font-variant-numeric: tabular-nums; }}
    .pill {{ font-weight: 600; }}
    .ok {{ color: var(--ok); }}
    .fail {{ color: var(--fail); }}
    .muted {{ color: var(--muted); }}
    .error {{ color: var(--fail); margin: .75rem 0 0; }}
    ul.endpoints, ul.sites {{ list-style: none; padding: 0; margin: 0; }}
    ul.endpoints li, ul.sites li {{
      display: flex; gap: .75rem; align-items: baseline;
      padding: .35rem 0; border-top: 1px solid var(--line);
    }}
    ul.endpoints li:first-child, ul.sites li:first-child {{ border-top: 0; padding-top: 0; }}
    code {{ font: 13px/1.4 ui-monospace, SFMono-Regular, Menlo, monospace; }}
    footer {{ color: var(--muted); font-size: .85rem; margin-top: 1.25rem; }}
  </style>
</head>
<body>
  <main>
    <h1>unifi-exporter</h1>
    <p class="sub">Read-only Prometheus exporter for UniFi Network · v{version}</p>

    <section class="card">
      <h2>Endpoints</h2>
      <ul class="endpoints">
        <li><a href="/metrics"><code>/metrics</code></a> <span class="muted">Prometheus text</span></li>
        <li><a href="/healthz"><code>/healthz</code></a> <span class="muted">process liveness</span></li>
        <li><a href="/readyz"><code>/readyz</code></a> <span class="muted">ready after first successful poll ({ready_label})</span></li>
      </ul>
    </section>

    <section class="card">
      <h2>Collector</h2>
      <dl>
        <dt>Poll</dt><dd><span class="pill {poll_class}">{poll_label}</span></dd>
        <dt>Integration</dt><dd><span class="pill {int_class}">{int_label}</span></dd>
        <dt>Classic</dt><dd><span class="pill {cls_class}">{cls_label}</span></dd>
        <dt>Cache age</dt><dd>{cache_age}</dd>
        <dt>Last poll</dt><dd>{poll_dur}</dd>
        <dt>Source</dt><dd>{source}</dd>
        <dt>Sites</dt><dd>{site_n}</dd>
        <dt>Devices</dt><dd>{devices}</dd>
        <dt>Clients</dt><dd>{clients}</dd>
      </dl>
      {error_block}
    </section>

    <section class="card">
      <h2>Sites</h2>
      {sites_block}
    </section>

    <footer>Session login is the only non-GET. Speedtest, Protect, and controller writes are not implemented.</footer>
  </main>
</body>
</html>
"##,
        poll_class = poll.0,
        poll_label = poll.1,
        int_class = integration.0,
        int_label = integration.1,
        cls_class = classic.0,
        cls_label = classic.1,
        ready_label = ready.1,
        source = escape(&snap.source),
        site_n = snap.sites.len(),
        devices = devices,
        clients = clients,
        cache_age = escape(&cache_age),
        poll_dur = escape(&poll_dur),
        error_block = error_block,
        sites_block = sites_block,
        version = escape(version),
    )
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::models::Site;
    use crate::poll::SiteSnapshot;

    #[test]
    fn landing_page_lists_endpoints_and_status() {
        let html = landing_html(&Snapshot {
            poll_ok: true,
            integration_ok: true,
            classic_ok: false,
            source: "udm.example".into(),
            sites: vec![SiteSnapshot {
                site: Site {
                    id: "s1".into(),
                    name: Some("Home".into()),
                    internal_reference: Some("default".into()),
                },
                ..Default::default()
            }],
            last_error: Some("classic: timeout".into()),
            ..Default::default()
        });
        assert!(html.contains("<title>unifi-exporter</title>"));
        assert!(html.contains("href=\"/metrics\""));
        assert!(html.contains("href=\"/healthz\""));
        assert!(html.contains("href=\"/readyz\""));
        assert!(html.contains("udm.example"));
        assert!(html.contains("Home"));
        assert!(html.contains("classic: timeout"));
        assert!(html.contains("not ready"));
    }

    #[test]
    fn landing_page_escapes_html() {
        let html = landing_html(&Snapshot {
            source: "<script>".into(),
            last_error: Some("a < b & c".into()),
            ..Default::default()
        });
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("a &lt; b &amp; c"));
    }
}
