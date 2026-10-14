use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

use unifi_exporter::client::{ClassicClientHttp, ReadOnlyClient};
use unifi_exporter::config::{Cli, Config};
use unifi_exporter::http::{router, AppState};
use unifi_exporter::metrics::Metrics;
use unifi_exporter::poll::{run_poller, SnapshotCache};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let cfg = Config::load(cli).context("invalid configuration")?;

    info!(
        mode = ?cfg.mode,
        listen = %cfg.listen,
        poll_interval_secs = cfg.poll_interval.as_secs(),
        tls_insecure = cfg.tls_insecure,
        integration = cfg.integration_enabled(),
        classic = cfg.classic_enabled(),
        "starting unifi-exporter (read-only hybrid)"
    );

    let integration = if cfg.integration_enabled() {
        Some(ReadOnlyClient::new(&cfg)?)
    } else {
        None
    };
    let classic = if cfg.classic_enabled() {
        Some(ClassicClientHttp::new(&cfg)?)
    } else {
        None
    };

    let cache = SnapshotCache::new();
    let metrics = Arc::new(Metrics::new()?);

    let poll_cache = cache.clone();
    let poll_metrics = metrics.clone();
    let interval = cfg.poll_interval;
    let concurrency = cfg.max_concurrency;
    let site = cfg.site.clone();
    let source = cfg.source_label();
    tokio::spawn(async move {
        run_poller(
            integration,
            classic,
            site,
            source,
            poll_cache,
            poll_metrics,
            interval,
            concurrency,
        )
        .await;
    });

    let app = router(AppState { cache, metrics });
    let listener = tokio::net::TcpListener::bind(cfg.listen)
        .await
        .with_context(|| format!("bind {}", cfg.listen))?;
    info!(addr = %cfg.listen, "listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    info!("shutdown signal received");
}
