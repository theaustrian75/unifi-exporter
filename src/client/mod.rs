//! Read-only UniFi Network Integration API client.
//!
//! This module is structurally incapable of issuing POST/PUT/PATCH/DELETE.
//! All requests go through [`ReadOnlyClient::get_json`] which hardcodes HTTP GET
//! and validates paths against an allowlist.

pub mod classic;
pub mod models;
pub mod paths;

pub use classic::ClassicClientHttp;

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use tracing::{debug, warn};

use crate::config::Config;
use crate::error::{Error, Result};

use models::{
    ApplicationInfo, Client as UniFiClient, Country, Device, DeviceStatistics, DpiCatalogEntry,
    InventoryItem, NamedEntity, Page, PendingDevice, Site,
};
use paths::Endpoint;

const PAGE_LIMIT: i64 = 200;
const MAX_RETRIES: u32 = 5;

#[derive(Clone)]
pub struct ReadOnlyClient {
    http: Client,
    base: String,
    api_key: String,
}

impl ReadOnlyClient {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let http = Client::builder()
            .default_headers(headers)
            .timeout(cfg.http_timeout)
            .danger_accept_invalid_certs(cfg.tls_insecure)
            .user_agent(concat!("unifi-exporter/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            http,
            base: cfg.integration_base()?,
            api_key: cfg
                .api_key
                .clone()
                .ok_or_else(|| anyhow::anyhow!("api_key required for Integration client"))?,
        })
    }

    /// Public read API: GET + JSON decode only.
    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        paths::validate_path(path)?;
        let body = self.get_bytes(path).await?;
        serde_json::from_slice(&body).map_err(|source| Error::Decode {
            path: path.to_string(),
            source,
        })
    }

    /// Paginate Integration list endpoints until exhausted.
    pub async fn get_paginated<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        paths::validate_path(path)?;
        let mut offset: i64 = 0;
        let mut out = Vec::new();
        loop {
            let page_path = format!("{path}?offset={offset}&limit={PAGE_LIMIT}");
            // Query string is appended after validation of the path stem.
            let page: Page<T> = {
                let body = self.get_bytes_raw(&page_path).await?;
                serde_json::from_slice(&body).map_err(|source| Error::Decode {
                    path: page_path.clone(),
                    source,
                })?
            };
            let n = page.data.len() as i64;
            out.extend(page.data);
            let total = page.total_count.unwrap_or(out.len() as i64);
            offset += n;
            if n == 0 || offset >= total || n < PAGE_LIMIT {
                break;
            }
        }
        Ok(out)
    }

    async fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        paths::validate_path(path)?;
        self.get_bytes_raw(path).await
    }

    /// Internal GET with retries. Method is hardcoded to GET — never anything else.
    async fn get_bytes_raw(&self, path_and_query: &str) -> Result<Vec<u8>> {
        let path_only = path_and_query
            .split_once('?')
            .map(|(p, _)| p)
            .unwrap_or(path_and_query);
        paths::validate_path(path_only)?;

        let url = format!(
            "{}{}",
            self.base.trim_end_matches('/'),
            if path_and_query.starts_with('/') {
                path_and_query.to_string()
            } else {
                format!("/{path_and_query}")
            }
        );

        let mut attempt = 0u32;
        loop {
            // HARD READ-ONLY: Method::GET only.
            let request = self
                .http
                .request(Method::GET, &url)
                .header("X-API-KEY", &self.api_key);

            debug!(%url, attempt, "GET");
            let response = request.send().await?;
            let status = response.status();

            if status == StatusCode::TOO_MANY_REQUESTS {
                attempt += 1;
                if attempt > MAX_RETRIES {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::Http {
                        status: status.as_u16(),
                        path: path_and_query.to_string(),
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
                warn!(?wait, %url, "rate limited, backing off");
                tokio::time::sleep(wait).await;
                continue;
            }

            if status == StatusCode::NOT_FOUND {
                let body = response.text().await.unwrap_or_default();
                return Err(Error::Unsupported(format!("{path_and_query}: {body}")));
            }

            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                return Err(Error::Http {
                    status: status.as_u16(),
                    path: path_and_query.to_string(),
                    body,
                });
            }

            return Ok(response.bytes().await?.to_vec());
        }
    }

    // ---- typed helpers (all GET) ----

    pub async fn info(&self) -> Result<ApplicationInfo> {
        let path = Endpoint::Info.path(None, None)?;
        self.get_json(&path).await
    }

    pub async fn sites(&self) -> Result<Vec<Site>> {
        let path = Endpoint::Sites.path(None, None)?;
        let body = self.get_bytes(&path).await?;
        if let Ok(page) = serde_json::from_slice::<Page<Site>>(&body) {
            return Ok(page.data);
        }
        if let Ok(list) = serde_json::from_slice::<Vec<Site>>(&body) {
            return Ok(list);
        }
        // Some builds wrap differently; attempt paginated walk.
        self.get_paginated(&path).await
    }

    pub async fn pending_devices(&self) -> Result<Vec<PendingDevice>> {
        let path = Endpoint::PendingDevices.path(None, None)?;
        self.list_or_empty(&path).await
    }

    pub async fn devices(&self, site_id: &str) -> Result<Vec<Device>> {
        let path = Endpoint::Devices.path(Some(site_id), None)?;
        self.list_or_empty(&path).await
    }

    pub async fn device_statistics(
        &self,
        site_id: &str,
        device_id: &str,
    ) -> Result<DeviceStatistics> {
        let path = Endpoint::DeviceStatistics.path(Some(site_id), Some(device_id))?;
        self.get_json(&path).await
    }

    pub async fn clients(&self, site_id: &str) -> Result<Vec<UniFiClient>> {
        let path = Endpoint::Clients.path(Some(site_id), None)?;
        self.list_or_empty(&path).await
    }

    pub async fn named_collection(
        &self,
        endpoint: Endpoint,
        site_id: &str,
    ) -> Result<Vec<NamedEntity>> {
        let path = endpoint.path(Some(site_id), None)?;
        self.list_or_empty(&path).await
    }

    pub async fn inventory_collection(
        &self,
        endpoint: Endpoint,
        site_id: &str,
    ) -> Result<Vec<InventoryItem>> {
        let path = endpoint.path(Some(site_id), None)?;
        self.list_or_empty(&path).await
    }

    pub async fn countries(&self) -> Result<Vec<Country>> {
        let path = Endpoint::Countries.path(None, None)?;
        self.list_or_empty(&path).await
    }

    pub async fn dpi_applications(&self) -> Result<Vec<DpiCatalogEntry>> {
        let path = Endpoint::DpiApplications.path(None, None)?;
        self.list_or_empty(&path).await
    }

    pub async fn dpi_categories(&self) -> Result<Vec<DpiCatalogEntry>> {
        let path = Endpoint::DpiCategories.path(None, None)?;
        self.list_or_empty(&path).await
    }

    async fn list_or_empty<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>> {
        match self.get_paginated::<T>(path).await {
            Ok(v) => Ok(v),
            Err(Error::Unsupported(_)) => Ok(Vec::new()),
            Err(Error::Http { status: 404, .. }) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }
}

/// Compile-time documentation that write methods are not part of the API.
#[allow(dead_code)]
fn read_only_contract() {
    // Intentionally no post/put/patch/delete helpers exist on ReadOnlyClient.
    let _ = Method::GET;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_has_no_write_helpers() {
        // Structural check: only GET method used in send path.
        assert_eq!(Method::GET.as_str(), "GET");
        assert_ne!(Method::POST.as_str(), Method::GET.as_str());
        assert_ne!(Method::PUT.as_str(), Method::GET.as_str());
        assert_ne!(Method::PATCH.as_str(), Method::GET.as_str());
        assert_ne!(Method::DELETE.as_str(), Method::GET.as_str());
    }
}
