use unifi_exporter::client::paths;

#[test]
fn rejects_write_and_classic_paths() {
    assert!(paths::validate_path("/v1/sites/x/devices/y/actions").is_err());
    assert!(paths::validate_path("/v1/sites/x/clients/y/actions").is_err());
    assert!(paths::validate_path("/v1/sites/x/firewall/policies/ordering").is_err());
    assert!(paths::validate_path("/v1/sites/x/networks/y/references").is_err());
    assert!(paths::validate_path("/api/auth/login").is_err());
    assert!(paths::validate_path("/v1/countries").is_ok());
    assert!(paths::validate_path("/v1/dpi/applications").is_ok());
}

#[test]
fn allows_integration_get_paths() {
    assert!(paths::validate_path("/v1/info").is_ok());
    assert!(paths::validate_path("/v1/sites").is_ok());
    assert!(paths::validate_path("/v1/pending-devices").is_ok());
    assert!(paths::validate_path("/v1/sites/abc-123/devices").is_ok());
    assert!(paths::validate_path("/v1/sites/abc-123/clients").is_ok());
    assert!(paths::validate_path(
        "/v1/sites/abc-123/devices/dev-1/statistics/latest"
    )
    .is_ok());
    assert!(paths::validate_path("/v1/sites/abc-123/wifi/broadcasts").is_ok());
    assert!(paths::validate_path("/v1/sites/abc-123/switching/lags").is_ok());
}

#[test]
fn endpoint_builder_never_emits_actions() {
    use unifi_exporter::client::paths::Endpoint;
    let p = Endpoint::Devices.path(Some("site-1"), None).unwrap();
    assert!(!p.contains("actions"));
    let p = Endpoint::DeviceStatistics
        .path(Some("site-1"), Some("dev-1"))
        .unwrap();
    assert!(p.ends_with("/statistics/latest"));
}
