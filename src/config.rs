use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use serde::Deserialize;

const DEFAULT_LISTEN: &str = "0.0.0.0:9130";
const DEFAULT_POLL_INTERVAL_SECS: u64 = 60;
const MIN_POLL_INTERVAL_SECS: u64 = 15;
const DEFAULT_HTTP_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_CONCURRENCY: usize = 8;
const DEFAULT_SITE: &str = "default";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[default]
    Local,
    Cloud,
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "unifi-exporter",
    about = "Read-only Prometheus exporter for UniFi (Integration + classic controller APIs)",
    version
)]
pub struct Cli {
    /// Optional TOML config file (defaults to ./config.toml when present)
    #[arg(short, long, env = "UNIFI_EXPORTER_CONFIG")]
    pub config: Option<PathBuf>,

    /// Connection mode: local console or Site Manager cloud connector
    #[arg(long, env = "UNIFI_EXPORTER_MODE", value_enum)]
    pub mode: Option<Mode>,

    /// Local console origin, e.g. https://192.168.1.1
    #[arg(long, env = "UNIFI_EXPORTER_BASE_URL")]
    pub base_url: Option<String>,

    /// UniFi Integration / Site Manager API key (X-API-KEY)
    #[arg(long, env = "UNIFI_EXPORTER_API_KEY")]
    pub api_key: Option<String>,

    /// Local UniFi OS username for classic API (view-only recommended)
    #[arg(long, env = "UNIFI_EXPORTER_USERNAME")]
    pub username: Option<String>,

    /// Local UniFi OS password for classic API
    #[arg(long, env = "UNIFI_EXPORTER_PASSWORD")]
    pub password: Option<String>,

    /// Classic API site name (internal reference), default "default"
    #[arg(long, env = "UNIFI_EXPORTER_SITE")]
    pub site: Option<String>,

    /// Console ID for cloud connector mode
    #[arg(long, env = "UNIFI_EXPORTER_CONSOLE_ID")]
    pub console_id: Option<String>,

    /// HTTP listen address for /metrics
    #[arg(long, env = "UNIFI_EXPORTER_LISTEN")]
    pub listen: Option<String>,

    /// Background poll interval (seconds)
    #[arg(long, env = "UNIFI_EXPORTER_POLL_INTERVAL_SECS")]
    pub poll_interval_secs: Option<u64>,

    /// Skip TLS certificate verification (true/false)
    #[arg(long, env = "UNIFI_EXPORTER_TLS_INSECURE")]
    pub tls_insecure: Option<bool>,

    /// Explicitly require TLS verification
    #[arg(long, env = "UNIFI_EXPORTER_TLS_VERIFY", action = clap::ArgAction::SetTrue)]
    pub tls_verify: bool,

    /// Per-request HTTP timeout (seconds)
    #[arg(long, env = "UNIFI_EXPORTER_HTTP_TIMEOUT_SECS")]
    pub http_timeout_secs: Option<u64>,

    /// Max concurrent UniFi API requests
    #[arg(long, env = "UNIFI_EXPORTER_MAX_CONCURRENCY")]
    pub max_concurrency: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct FileConfig {
    pub mode: Option<Mode>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub site: Option<String>,
    pub console_id: Option<String>,
    pub listen: Option<String>,
    pub poll_interval_secs: Option<u64>,
    pub tls_insecure: Option<bool>,
    pub http_timeout_secs: Option<u64>,
    pub max_concurrency: Option<usize>,
}

#[derive(Clone)]
pub struct Config {
    pub mode: Mode,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub site: String,
    pub console_id: Option<String>,
    pub listen: SocketAddr,
    pub poll_interval: Duration,
    pub tls_insecure: bool,
    pub http_timeout: Duration,
    pub max_concurrency: usize,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("mode", &self.mode)
            .field("base_url", &self.base_url)
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .field("username", &self.username)
            .field("password", &self.password.as_ref().map(|_| "***"))
            .field("site", &self.site)
            .field("console_id", &self.console_id)
            .field("listen", &self.listen)
            .field("poll_interval", &self.poll_interval)
            .field("tls_insecure", &self.tls_insecure)
            .field("http_timeout", &self.http_timeout)
            .field("max_concurrency", &self.max_concurrency)
            .finish()
    }
}

impl Config {
    pub fn load(cli: Cli) -> Result<Self> {
        let config_path = cli.config.clone().or_else(default_config_path);
        let file = if let Some(path) = &config_path {
            let text = fs::read_to_string(path)
                .with_context(|| format!("failed to read config {}", path.display()))?;
            toml::from_str::<FileConfig>(&text)
                .with_context(|| format!("failed to parse config {}", path.display()))?
        } else {
            FileConfig::default()
        };

        let mode = cli.mode.or(file.mode).unwrap_or_default();
        let api_key = nonempty(cli.api_key.or(file.api_key));
        let username = nonempty(cli.username.or(file.username));
        let password = nonempty(cli.password.or(file.password));
        let site = nonempty(cli.site.or(file.site)).unwrap_or_else(|| DEFAULT_SITE.to_string());

        let base_url = nonempty(cli.base_url.or(file.base_url));
        let console_id = nonempty(cli.console_id.or(file.console_id));

        let integration_ok = api_key.is_some();
        let classic_ok = username.is_some() && password.is_some() && base_url.is_some();
        if !integration_ok && !classic_ok {
            bail!(
                "at least one source is required: Integration (api_key) and/or classic (username+password+base_url)"
            );
        }

        match mode {
            Mode::Local => {
                if base_url.is_none() {
                    bail!("base_url is required in local mode");
                }
            }
            Mode::Cloud => {
                if integration_ok && console_id.is_none() {
                    bail!("console_id is required for Integration cloud mode");
                }
                if classic_ok && base_url.is_none() {
                    bail!("base_url is required for classic API (local console)");
                }
            }
        }

        let listen_str = cli
            .listen
            .or(file.listen)
            .unwrap_or_else(|| DEFAULT_LISTEN.to_string());
        let listen: SocketAddr = listen_str
            .parse()
            .with_context(|| format!("invalid listen address: {listen_str}"))?;

        let mut poll_secs = cli
            .poll_interval_secs
            .or(file.poll_interval_secs)
            .unwrap_or(DEFAULT_POLL_INTERVAL_SECS);
        if poll_secs < MIN_POLL_INTERVAL_SECS {
            poll_secs = MIN_POLL_INTERVAL_SECS;
        }

        let tls_insecure = if cli.tls_verify {
            false
        } else if let Some(v) = cli.tls_insecure {
            v
        } else if let Some(v) = file.tls_insecure {
            v
        } else {
            matches!(mode, Mode::Local) || classic_ok
        };

        let http_timeout_secs = cli
            .http_timeout_secs
            .or(file.http_timeout_secs)
            .unwrap_or(DEFAULT_HTTP_TIMEOUT_SECS);

        let max_concurrency = cli
            .max_concurrency
            .or(file.max_concurrency)
            .unwrap_or(DEFAULT_MAX_CONCURRENCY)
            .max(1);

        Ok(Self {
            mode,
            base_url,
            api_key,
            username,
            password,
            site,
            console_id,
            listen,
            poll_interval: Duration::from_secs(poll_secs),
            tls_insecure,
            http_timeout: Duration::from_secs(http_timeout_secs),
            max_concurrency,
        })
    }

    pub fn integration_enabled(&self) -> bool {
        self.api_key.as_ref().is_some_and(|s| !s.is_empty())
    }

    pub fn classic_enabled(&self) -> bool {
        self.username.as_ref().is_some_and(|s| !s.is_empty())
            && self.password.as_ref().is_some_and(|s| !s.is_empty())
            && self.base_url.as_ref().is_some_and(|s| !s.is_empty())
    }

    /// Integration API root (no trailing slash), e.g. https://host/proxy/network/integration
    pub fn integration_base(&self) -> Result<String> {
        match self.mode {
            Mode::Local => {
                let origin = self
                    .base_url
                    .as_ref()
                    .context("base_url missing")?
                    .trim_end_matches('/');
                Ok(format!("{origin}/proxy/network/integration"))
            }
            Mode::Cloud => {
                let id = self
                    .console_id
                    .as_ref()
                    .context("console_id missing")?
                    .trim();
                Ok(format!(
                    "https://api.ui.com/v1/connector/consoles/{id}/proxy/network/integration"
                ))
            }
        }
    }

    pub fn origin(&self) -> Result<String> {
        Ok(self
            .base_url
            .as_ref()
            .context("base_url missing")?
            .trim_end_matches('/')
            .to_string())
    }

    pub fn source_label(&self) -> String {
        self.base_url
            .as_deref()
            .map(host_from_url)
            .unwrap_or_else(|| "unifi-exporter".into())
    }
}

fn nonempty(v: Option<String>) -> Option<String> {
    v.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() {
            None
        } else {
            Some(t)
        }
    })
}

fn host_from_url(raw: &str) -> String {
    let s = raw.trim().trim_end_matches('/');
    let s = s
        .strip_prefix("https://")
        .or_else(|| s.strip_prefix("http://"))
        .unwrap_or(s);
    s.split('/')
        .next()
        .unwrap_or("unifi-exporter")
        .split(':')
        .next()
        .unwrap_or("unifi-exporter")
        .to_string()
}

/// Prefer an explicit path; otherwise load `./config.toml` when it exists.
fn default_config_path() -> Option<PathBuf> {
    let path = PathBuf::from("config.toml");
    path.is_file().then_some(path)
}
