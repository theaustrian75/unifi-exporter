use wiremock::matchers::{method, path as path_matcher};
use wiremock::{Mock, MockServer, ResponseTemplate};

use unifi_exporter::client::classic::paths;
use unifi_exporter::client::ClassicClientHttp;
use unifi_exporter::config::{Config, Mode};

#[test]
fn classic_allowlist_rejects_cmd_and_actions() {
    assert!(paths::validate_get_path("/proxy/network/api/s/default/cmd/devmgr").is_err());
    assert!(paths::validate_get_path("/proxy/network/api/s/default/stat/device/actions").is_err());
    assert!(paths::validate_get_path("/api/auth/login").is_err());
    assert!(paths::stat_path("default", "sta").is_ok());
    assert!(paths::stat_path("default", "ups-devices").is_ok());
    assert!(paths::rest_path("default", "portforward").is_ok());
    assert!(paths::v2_path("default", "active-leases").is_ok());
    assert!(paths::drive_path("storage").is_ok());
}

#[tokio::test]
async fn classic_login_then_get_sta() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path_matcher("/api/auth/login"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("set-cookie", "TOKEN=test-token; Path=/; HttpOnly")
                .insert_header("x-csrf-token", "csrf-1")
                .set_body_json(serde_json::json!({"unique_id": "u1"})),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_matcher("/proxy/network/api/s/default/stat/sta"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "meta": {"rc": "ok"},
            "data": [{
                "mac": "11:22:33:44:55:66",
                "name": "phone",
                "rssi": -60,
                "is_wired": false
            }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let cfg = Config {
        mode: Mode::Local,
        base_url: Some(server.uri()),
        api_key: None,
        username: Some("viewer".into()),
        password: Some("secret".into()),
        site: "default".into(),
        console_id: None,
        listen: "127.0.0.1:0".parse().unwrap(),
        poll_interval: std::time::Duration::from_secs(60),
        tls_insecure: true,
        http_timeout: std::time::Duration::from_secs(5),
        max_concurrency: 2,
    };

    let client = ClassicClientHttp::new(&cfg).unwrap();
    let clients = client.clients("default").await.unwrap();
    assert_eq!(clients.len(), 1);
    assert_eq!(clients[0].name.as_deref(), Some("phone"));
    assert_eq!(clients[0].rssi.get(), Some(-60.0));
}
