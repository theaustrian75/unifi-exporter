//! Allowlisted GET paths for the UniFi Network Integration API.
//! Only these path templates may be requested. Write verbs are never used.

use crate::error::{Error, Result};

/// Known Integration API GET resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endpoint {
    Info,
    Sites,
    PendingDevices,
    Devices,
    DeviceStatistics,
    Clients,
    Networks,
    WifiBroadcasts,
    Wans,
    VpnServers,
    VpnSiteToSiteTunnels,
    DeviceTags,
    RadiusProfiles,
    FirewallPolicies,
    FirewallZones,
    AclRules,
    DnsPolicies,
    TrafficMatchingLists,
    HotspotVouchers,
    SwitchingLags,
    SwitchingMcLagDomains,
    SwitchingSwitchStacks,
    Countries,
    DpiApplications,
    DpiCategories,
}

impl Endpoint {
    pub fn path(self, site_id: Option<&str>, device_id: Option<&str>) -> Result<String> {
        let path = match self {
            Self::Info => "/v1/info".to_string(),
            Self::Sites => "/v1/sites".to_string(),
            Self::PendingDevices => "/v1/pending-devices".to_string(),
            Self::Devices => format!("/v1/sites/{}/devices", need_site(site_id)?),
            Self::DeviceStatistics => format!(
                "/v1/sites/{}/devices/{}/statistics/latest",
                need_site(site_id)?,
                need_device(device_id)?
            ),
            Self::Clients => format!("/v1/sites/{}/clients", need_site(site_id)?),
            Self::Networks => format!("/v1/sites/{}/networks", need_site(site_id)?),
            Self::WifiBroadcasts => format!("/v1/sites/{}/wifi/broadcasts", need_site(site_id)?),
            Self::Wans => format!("/v1/sites/{}/wans", need_site(site_id)?),
            Self::VpnServers => format!("/v1/sites/{}/vpn/servers", need_site(site_id)?),
            Self::VpnSiteToSiteTunnels => {
                format!("/v1/sites/{}/vpn/site-to-site-tunnels", need_site(site_id)?)
            }
            Self::DeviceTags => format!("/v1/sites/{}/device-tags", need_site(site_id)?),
            Self::RadiusProfiles => format!("/v1/sites/{}/radius/profiles", need_site(site_id)?),
            Self::FirewallPolicies => {
                format!("/v1/sites/{}/firewall/policies", need_site(site_id)?)
            }
            Self::FirewallZones => format!("/v1/sites/{}/firewall/zones", need_site(site_id)?),
            Self::AclRules => format!("/v1/sites/{}/acl-rules", need_site(site_id)?),
            Self::DnsPolicies => format!("/v1/sites/{}/dns/policies", need_site(site_id)?),
            Self::TrafficMatchingLists => {
                format!("/v1/sites/{}/traffic-matching-lists", need_site(site_id)?)
            }
            Self::HotspotVouchers => {
                format!("/v1/sites/{}/hotspot/vouchers", need_site(site_id)?)
            }
            Self::SwitchingLags => format!("/v1/sites/{}/switching/lags", need_site(site_id)?),
            Self::SwitchingMcLagDomains => {
                format!("/v1/sites/{}/switching/mc-lag-domains", need_site(site_id)?)
            }
            Self::SwitchingSwitchStacks => {
                format!("/v1/sites/{}/switching/switch-stacks", need_site(site_id)?)
            }
            Self::Countries => "/v1/countries".to_string(),
            Self::DpiApplications => "/v1/dpi/applications".to_string(),
            Self::DpiCategories => "/v1/dpi/categories".to_string(),
        };
        validate_path(&path)?;
        Ok(path)
    }
}

fn need_site(site_id: Option<&str>) -> Result<&str> {
    site_id
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::Message("site_id required".into()))
}

fn need_device(device_id: Option<&str>) -> Result<&str> {
    device_id
        .filter(|s| !s.is_empty())
        .ok_or_else(|| Error::Message("device_id required".into()))
}

/// Reject anything that is not an allowlisted Integration GET path shape.
pub fn validate_path(path: &str) -> Result<()> {
    if !path.starts_with("/v1/") {
        return Err(Error::PathNotAllowed(path.to_string()));
    }
    // Hard ban action / mutation URL fragments even if somehow constructed.
    let lower = path.to_ascii_lowercase();
    for banned in [
        "/actions",
        "/ordering",
        "/references",
        "/api/auth",
        "/proxy/network/api/",
    ] {
        if lower.contains(banned) {
            return Err(Error::PathNotAllowed(path.to_string()));
        }
    }

    let allowed = path_matches_allowlist(path);
    if !allowed {
        return Err(Error::PathNotAllowed(path.to_string()));
    }
    Ok(())
}

fn path_matches_allowlist(path: &str) -> bool {
    // Exact globals
    matches!(
        path,
        "/v1/info"
            | "/v1/sites"
            | "/v1/pending-devices"
            | "/v1/countries"
            | "/v1/dpi/applications"
            | "/v1/dpi/categories"
    ) || is_site_collection(path)
        || is_device_statistics(path)
}

fn is_site_collection(path: &str) -> bool {
    // /v1/sites/{uuid}/...
    let Some(rest) = path.strip_prefix("/v1/sites/") else {
        return false;
    };
    let Some((site, tail)) = rest.split_once('/') else {
        return false;
    };
    if !is_safe_id(site) {
        return false;
    }
    matches!(
        tail,
        "devices"
            | "clients"
            | "networks"
            | "wifi/broadcasts"
            | "wans"
            | "vpn/servers"
            | "vpn/site-to-site-tunnels"
            | "device-tags"
            | "radius/profiles"
            | "firewall/policies"
            | "firewall/zones"
            | "acl-rules"
            | "dns/policies"
            | "traffic-matching-lists"
            | "hotspot/vouchers"
            | "switching/lags"
            | "switching/mc-lag-domains"
            | "switching/switch-stacks"
    )
}

fn is_device_statistics(path: &str) -> bool {
    // /v1/sites/{site}/devices/{device}/statistics/latest
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
    matches!(
        parts.as_slice(),
        ["v1", "sites", site, "devices", device, "statistics", "latest"]
            if is_safe_id(site) && is_safe_id(device)
    )
}

fn is_safe_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_known_paths() {
        assert!(validate_path("/v1/info").is_ok());
        assert!(validate_path("/v1/sites").is_ok());
        assert!(validate_path("/v1/sites/abc-123/devices").is_ok());
        assert!(validate_path(
            "/v1/sites/abc-123/devices/dev-1/statistics/latest"
        )
        .is_ok());
        assert!(validate_path("/v1/countries").is_ok());
        assert!(validate_path("/v1/dpi/applications").is_ok());
        assert!(validate_path("/v1/dpi/categories").is_ok());
    }

    #[test]
    fn rejects_writes_and_classic() {
        assert!(validate_path("/v1/sites/x/devices/y/actions").is_err());
        assert!(validate_path("/v1/sites/x/firewall/policies/ordering").is_err());
        assert!(validate_path("/proxy/network/api/s/default/stat/device").is_err());
        assert!(validate_path("/api/auth/login").is_err());
    }
}
