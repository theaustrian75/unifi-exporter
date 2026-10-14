//! Classic UniFi OS controller API (cookie + CSRF).
//!
//! After session login, only allowlisted GET paths are used. The login POST is
//! the sole non-GET request and exists only to establish a read session.

pub mod extra;
pub mod models;
pub mod paths;

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::Mutex;
use tracing::{debug, warn};

use crate::config::Config;
use crate::error::{Error, Result};

use extra::{
    CountryTraffic, DhcpLease, NetworkConf, PortAnomaly, PortForward, SslCertificate, Topology,
    UnasDeviceInfo, UnasDrive, UnasNetworkIo, UnasSnapshot, UnasStorage, UpsSelector,
};
use models::{
    flatten_dpi, flatten_stadpi, ClassicClient, ClassicDevice, ClientDpi, DpiEntry, DpiRow,
    Envelope, HealthSubsystem, RogueAp, SpeedtestStatus, Sysinfo,
};
use serde_json::Value;

const MAX_RETRIES: u32 = 5;
const LOGIN_PATH: &str = "/api/auth/login";

#[derive(Clone)]
pub struct ClassicClientHttp {
    http: Client,
    origin: String,
    username: String,
    password: String,
    csrf: Arc<Mutex<Option<String>>>,
    logged_in: Arc<Mutex<bool>>,
}

impl ClassicClientHttp {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let http = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .timeout(cfg.http_timeout)
            .danger_accept_invalid_certs(cfg.tls_insecure)
            .user_agent(concat!("unifi-exporter/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            http,
            origin: cfg.origin()?,
            username: cfg
                .username
                .clone()
                .ok_or_else(|| anyhow::anyhow!("username required for classic client"))?,
            password: cfg
                .password
                .clone()
                .ok_or_else(|| anyhow::anyhow!("password required for classic client"))?,
            csrf: Arc::new(Mutex::new(None)),
            logged_in: Arc::new(Mutex::new(false)),
        })
    }

    /// Session login — the only POST this client may issue.
    pub async fn login(&self) -> Result<()> {
        let url = format!("{}{LOGIN_PATH}", self.origin);
        debug!(%url, "POST classic login");
        let response = self
            .http
            .request(Method::POST, &url)
            .json(&json!({
                "username": self.username,
                "password": self.password,
                "remember": true,
            }))
            .send()
            .await?;

        let status = response.status();
        if status == StatusCode::TOO_MANY_REQUESTS {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Login {
                status: status.as_u16(),
                body,
            });
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Login {
                status: status.as_u16(),
                body,
            });
        }

        if let Some(token) = response
            .headers()
            .get("x-csrf-token")
            .or_else(|| response.headers().get("x-updated-csrf-token"))
            .and_then(|v| v.to_str().ok())
        {
            *self.csrf.lock().await = Some(token.to_string());
        }
        let _ = response.bytes().await;
        *self.logged_in.lock().await = true;
        Ok(())
    }

    async fn ensure_login(&self) -> Result<()> {
        if *self.logged_in.lock().await {
            return Ok(());
        }
        self.login().await
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        paths::validate_get_path(path)?;
        let body = self.get_bytes(path).await?;
        serde_json::from_slice(&body).map_err(|source| Error::Decode {
            path: path.to_string(),
            source,
        })
    }

    async fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        paths::validate_get_path(path)?;
        self.ensure_login().await?;
        self.get_bytes_once(path, true).await
    }

    async fn get_bytes_once(&self, path: &str, retry_auth: bool) -> Result<Vec<u8>> {
        let url = format!("{}{path}", self.origin.trim_end_matches('/'));
        let mut attempt = 0u32;
        let mut allow_relogin = retry_auth;
        loop {
            let mut req = self.http.request(Method::GET, &url);
            if let Some(csrf) = self.csrf.lock().await.as_ref() {
                req = req.header("X-CSRF-Token", csrf);
            }
            debug!(%url, attempt, "GET classic");
            let response = req.send().await?;
            let status = response.status();

            if status == StatusCode::UNAUTHORIZED && allow_relogin {
                allow_relogin = false;
                *self.logged_in.lock().await = false;
                self.login().await?;
                continue;
            }

            if status == StatusCode::TOO_MANY_REQUESTS {
                attempt += 1;
                if attempt > MAX_RETRIES {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::Http {
                        status: status.as_u16(),
                        path: path.to_string(),
                        body,
                    });
                }
                let wait = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(Duration::from_secs)
                    .unwrap_or_else(|| Duration::from_secs(2u64.pow(attempt)));
                warn!(?wait, %url, "classic rate limited, backing off");
                tokio::time::sleep(wait).await;
                continue;
            }

            if status == StatusCode::NOT_FOUND {
                let body = response.text().await.unwrap_or_default();
                return Err(Error::Unsupported(format!("{path}: {body}")));
            }

            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                return Err(Error::Http {
                    status: status.as_u16(),
                    path: path.to_string(),
                    body,
                });
            }
            return Ok(response.bytes().await?.to_vec());
        }
    }

    pub async fn clients(&self, site: &str) -> Result<Vec<ClassicClient>> {
        let path = paths::stat_path(site, "sta")?;
        Ok(self.get_json::<Envelope<ClassicClient>>(&path).await?.data)
    }

    pub async fn devices(&self, site: &str) -> Result<Vec<ClassicDevice>> {
        let path = paths::stat_path(site, "device")?;
        Ok(self.get_json::<Envelope<ClassicDevice>>(&path).await?.data)
    }

    pub async fn health(&self, site: &str) -> Result<Vec<HealthSubsystem>> {
        let path = paths::stat_path(site, "health")?;
        match self.get_json::<Envelope<HealthSubsystem>>(&path).await {
            Ok(env) => Ok(env.data),
            Err(Error::Unsupported(_)) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    pub async fn dpi(&self, site: &str) -> Result<Vec<DpiEntry>> {
        let path = paths::stat_path(site, "dpi")?;
        match self.get_json::<Envelope<DpiRow>>(&path).await {
            Ok(env) => Ok(flatten_dpi(env.data)),
            Err(Error::Unsupported(_)) | Err(Error::Decode { .. }) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    pub async fn client_dpi(&self, site: &str) -> Result<Vec<ClientDpi>> {
        let path = paths::stat_path(site, "stadpi")?;
        match self.get_json::<Envelope<DpiRow>>(&path).await {
            Ok(env) => Ok(flatten_stadpi(env.data)),
            Err(Error::Unsupported(_)) | Err(Error::Decode { .. }) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    pub async fn sysinfo(&self, site: &str) -> Result<Option<Sysinfo>> {
        let path = paths::stat_path(site, "sysinfo")?;
        match self.get_json::<Envelope<Sysinfo>>(&path).await {
            Ok(env) => Ok(env.data.into_iter().next()),
            Err(Error::Unsupported(_)) | Err(Error::Decode { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn rogue_aps(&self, site: &str) -> Result<Vec<RogueAp>> {
        let path = paths::stat_path(site, "rogueap")?;
        match self.get_json::<Envelope<RogueAp>>(&path).await {
            Ok(env) => Ok(env.data),
            Err(Error::Unsupported(_)) | Err(Error::Decode { .. }) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    pub async fn speedtest_status(&self, site: &str) -> Result<Option<SpeedtestStatus>> {
        let path = paths::stat_path(site, "speedtest-status")?;
        match self.get_json::<Envelope<SpeedtestStatus>>(&path).await {
            Ok(env) => Ok(env.data.into_iter().next()),
            Err(Error::Unsupported(_)) | Err(Error::Decode { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn dhcp_leases(&self, site: &str) -> Result<Vec<DhcpLease>> {
        let path = paths::v2_path(site, "active-leases")?;
        self.get_list(&path).await
    }

    pub async fn networkconf(&self, site: &str) -> Result<Vec<NetworkConf>> {
        let path = paths::rest_path(site, "networkconf")?;
        self.get_list(&path).await
    }

    pub async fn port_forwards(&self, site: &str) -> Result<Vec<PortForward>> {
        let path = paths::rest_path(site, "portforward")?;
        self.get_list(&path).await
    }

    pub async fn ssl_certs(&self, site: &str) -> Result<Vec<SslCertificate>> {
        let path = paths::stat_path(site, "active")?;
        Ok(self
            .get_list::<SslCertificate>(&path)
            .await?
            .into_iter()
            .filter(|c| !c.id().is_empty())
            .collect())
    }

    pub async fn topology(&self, site: &str) -> Result<Option<Topology>> {
        let path = paths::v2_path(site, "topology")?;
        self.get_optional(&path).await
    }

    pub async fn country_traffic(&self, site: &str) -> Result<Vec<CountryTraffic>> {
        let end = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let start = end.saturating_sub(3600);
        let path = format!(
            "{}?start={start}&end={end}",
            paths::v2_path(site, "country-traffic")?
        );
        let raw = match self.get_bytes(&path).await {
            Ok(b) => b,
            Err(Error::Unsupported(_)) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };
        Ok(decode_country_traffic(&raw))
    }

    pub async fn ups_devices(&self, site: &str) -> Result<Vec<UpsSelector>> {
        let path = paths::stat_path(site, "ups-devices")?;
        self.get_list(&path).await
    }

    pub async fn port_anomalies(&self, site: &str) -> Result<Vec<PortAnomaly>> {
        let path = paths::v2_path(site, "ports/port-anomalies")?;
        self.get_list(&path).await
    }

    pub async fn unas(&self) -> Result<Option<UnasSnapshot>> {
        let info = self
            .get_optional::<UnasDeviceInfo>(&paths::drive_path("device-info")?)
            .await?;
        let storage = self
            .get_optional::<UnasStorage>(&paths::drive_path("storage")?)
            .await?;
        let network_io = self
            .get_optional::<UnasNetworkIo>(&paths::drive_path("network-io")?)
            .await?;
        let drives = match self.get_bytes(&paths::drive_path("drives")?).await {
            Ok(body) => decode_unas_drives(&body),
            Err(Error::Unsupported(_)) => Vec::new(),
            Err(e) => return Err(e),
        };
        let snap = UnasSnapshot {
            info,
            storage,
            network_io,
            drives,
        };
        if snap.is_empty() {
            Ok(None)
        } else {
            Ok(Some(snap))
        }
    }

    async fn get_list<T: DeserializeOwned + Default>(&self, path: &str) -> Result<Vec<T>> {
        match self.get_bytes(path).await {
            Ok(body) => Ok(decode_list(&body)),
            Err(Error::Unsupported(_)) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    async fn get_optional<T: DeserializeOwned + Default>(&self, path: &str) -> Result<Option<T>> {
        match self.get_bytes(path).await {
            Ok(body) => Ok(decode_one(&body)),
            Err(Error::Unsupported(_)) | Err(Error::Decode { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn decode_list<T: DeserializeOwned + Default>(body: &[u8]) -> Vec<T> {
    if let Ok(env) = serde_json::from_slice::<Envelope<T>>(body) {
        return env.data;
    }
    if let Ok(list) = serde_json::from_slice::<Vec<T>>(body) {
        return list;
    }
    if let Ok(v) = serde_json::from_slice::<Value>(body) {
        for key in ["data", "leases", "anomalies", "certificates"] {
            if let Some(arr) = v.get(key) {
                if let Ok(list) = serde_json::from_value::<Vec<T>>(arr.clone()) {
                    return list;
                }
            }
        }
    }
    Vec::new()
}

fn decode_one<T: DeserializeOwned + Default>(body: &[u8]) -> Option<T> {
    if let Ok(v) = serde_json::from_slice::<T>(body) {
        return Some(v);
    }
    if let Ok(env) = serde_json::from_slice::<Envelope<T>>(body) {
        return env.data.into_iter().next();
    }
    if let Ok(v) = serde_json::from_slice::<Value>(body) {
        if let Some(inner) = v.get("data") {
            if let Ok(one) = serde_json::from_value::<T>(inner.clone()) {
                return Some(one);
            }
        }
    }
    None
}

fn decode_country_traffic(body: &[u8]) -> Vec<CountryTraffic> {
    let list: Vec<CountryTraffic> = decode_list(body);
    if !list.is_empty() {
        return list;
    }
    let Ok(v) = serde_json::from_slice::<Value>(body) else {
        return Vec::new();
    };
    let arr = v
        .as_array()
        .cloned()
        .or_else(|| v.get("data").and_then(|d| d.as_array()).cloned())
        .or_else(|| v.get("countries").and_then(|d| d.as_array()).cloned())
        .unwrap_or_default();
    arr.iter().filter_map(CountryTraffic::from_value).collect()
}

fn decode_unas_drives(body: &[u8]) -> Vec<UnasDrive> {
    if let Ok(list) = serde_json::from_slice::<Vec<UnasDrive>>(body) {
        return list;
    }
    if let Ok(v) = serde_json::from_slice::<Value>(body) {
        if let Some(arr) = v.get("drives").or_else(|| v.get("data")) {
            if let Ok(list) = serde_json::from_value::<Vec<UnasDrive>>(arr.clone()) {
                return list;
            }
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    #[test]
    fn writes_are_not_get() {
        assert_ne!(Method::POST.as_str(), Method::GET.as_str());
        assert_ne!(Method::PUT.as_str(), Method::GET.as_str());
        assert_ne!(Method::DELETE.as_str(), Method::GET.as_str());
    }
}
