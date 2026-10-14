//! Allowlisted classic UniFi OS GET paths.
//! The only permitted POST is `/api/auth/login`, invoked solely by the login helper.

use crate::error::{Error, Result};

const STAT_RESOURCES: &[&str] = &[
    "sta",
    "device",
    "health",
    "sysinfo",
    "dpi",
    "stadpi",
    "rogueap",
    "speedtest-status",
    "ups-devices",
    "active",
];

const REST_RESOURCES: &[&str] = &["portforward", "networkconf"];

const V2_RESOURCES: &[&str] = &[
    "active-leases",
    "topology",
    "country-traffic",
    "ports/port-anomalies",
];

const DRIVE_PATHS: &[&str] = &[
    "/proxy/drive/api/v2/systems/device-info",
    "/proxy/drive/api/v2/storage",
    "/proxy/drive/api/v2/systems/network-io",
    "/proxy/users/drive/api/v2/drives",
];

pub fn stat_path(site: &str, resource: &str) -> Result<String> {
    require_safe_id(site)?;
    if !STAT_RESOURCES.contains(&resource) {
        return Err(Error::PathNotAllowed(resource.to_string()));
    }
    let path = format!("/proxy/network/api/s/{site}/stat/{resource}");
    validate_get_path(&path)?;
    Ok(path)
}

pub fn rest_path(site: &str, resource: &str) -> Result<String> {
    require_safe_id(site)?;
    if !REST_RESOURCES.contains(&resource) {
        return Err(Error::PathNotAllowed(resource.to_string()));
    }
    let path = format!("/proxy/network/api/s/{site}/rest/{resource}");
    validate_get_path(&path)?;
    Ok(path)
}

pub fn v2_path(site: &str, resource: &str) -> Result<String> {
    require_safe_id(site)?;
    if !V2_RESOURCES.contains(&resource) {
        return Err(Error::PathNotAllowed(resource.to_string()));
    }
    let path = format!("/proxy/network/v2/api/site/{site}/{resource}");
    validate_get_path(&path)?;
    Ok(path)
}

pub fn drive_path(kind: &str) -> Result<String> {
    let path = match kind {
        "device-info" => DRIVE_PATHS[0],
        "storage" => DRIVE_PATHS[1],
        "network-io" => DRIVE_PATHS[2],
        "drives" => DRIVE_PATHS[3],
        other => return Err(Error::PathNotAllowed(other.to_string())),
    };
    validate_get_path(path)?;
    Ok(path.to_string())
}

pub fn validate_get_path(path: &str) -> Result<()> {
    let path = path.split('?').next().unwrap_or(path);
    let lower = path.to_ascii_lowercase();
    for banned in ["/cmd/", "/actions", "/api/auth/login", "/api/auth/logout"] {
        if lower.contains(banned) {
            return Err(Error::PathNotAllowed(path.to_string()));
        }
    }
    if lower.contains("/integration/") {
        return Err(Error::PathNotAllowed(path.to_string()));
    }
    if DRIVE_PATHS.contains(&path) {
        return Ok(());
    }
    if let Some(rest) = path.strip_prefix("/proxy/network/api/s/") {
        return validate_legacy(rest);
    }
    if let Some(rest) = path.strip_prefix("/proxy/network/v2/api/site/") {
        return validate_v2(rest);
    }
    Err(Error::PathNotAllowed(path.to_string()))
}

fn validate_legacy(rest: &str) -> Result<()> {
    let Some((site, tail)) = rest.split_once('/') else {
        return Err(Error::PathNotAllowed(rest.to_string()));
    };
    if !is_safe_id(site) {
        return Err(Error::PathNotAllowed(rest.to_string()));
    }
    if let Some(resource) = tail.strip_prefix("stat/") {
        if STAT_RESOURCES.contains(&resource) {
            return Ok(());
        }
    }
    if let Some(resource) = tail.strip_prefix("rest/") {
        if REST_RESOURCES.contains(&resource) {
            return Ok(());
        }
    }
    Err(Error::PathNotAllowed(rest.to_string()))
}

fn validate_v2(rest: &str) -> Result<()> {
    let Some((site, resource)) = rest.split_once('/') else {
        return Err(Error::PathNotAllowed(rest.to_string()));
    };
    if !is_safe_id(site) {
        return Err(Error::PathNotAllowed(rest.to_string()));
    }
    if V2_RESOURCES.contains(&resource) {
        return Ok(());
    }
    Err(Error::PathNotAllowed(rest.to_string()))
}

fn require_safe_id(id: &str) -> Result<()> {
    if is_safe_id(id) {
        Ok(())
    } else {
        Err(Error::PathNotAllowed(format!("invalid site {id}")))
    }
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
    fn allows_stat_gets() {
        assert!(validate_get_path("/proxy/network/api/s/default/stat/sta").is_ok());
        assert!(validate_get_path("/proxy/network/api/s/default/stat/device").is_ok());
        assert!(validate_get_path("/proxy/network/api/s/default/stat/ups-devices").is_ok());
        assert!(validate_get_path("/proxy/network/api/s/default/rest/portforward").is_ok());
        assert!(validate_get_path("/proxy/network/v2/api/site/default/active-leases").is_ok());
        assert!(validate_get_path(
            "/proxy/network/v2/api/site/default/country-traffic?start=1&end=2"
        )
        .is_ok());
        assert!(validate_get_path("/proxy/drive/api/v2/storage").is_ok());
        assert!(validate_get_path("/proxy/drive/api/v2/systems/network-io").is_ok());
        assert!(validate_get_path("/proxy/users/drive/api/v2/drives").is_ok());
    }

    #[test]
    fn rejects_writes_and_cmd() {
        assert!(validate_get_path("/proxy/network/api/s/default/cmd/devmgr").is_err());
        assert!(validate_get_path("/api/auth/login").is_err());
        assert!(validate_get_path("/proxy/network/api/s/default/stat/sta/actions").is_err());
        assert!(stat_path("default", "restart").is_err());
        assert!(rest_path("default", "user").is_err());
    }
}
