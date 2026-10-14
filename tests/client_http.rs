use wiremock::matchers::{header, method, path as path_matcher};
use wiremock::{Mock, MockServer, ResponseTemplate};

use unifi_exporter::client::ReadOnlyClient;
use unifi_exporter::config::{Config, Mode};

#[tokio::test]
async fn get_info_uses_get_and_api_key() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_matcher("/proxy/network/integration/v1/info"))
        .and(header("X-API-KEY", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "applicationVersion": "10.4.57"
        })))
        .mount(&server)
        .await;

    // Reject any non-GET just in case
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let cfg = Config {
        mode: Mode::Local,
        base_url: Some(server.uri()),
        api_key: Some("test-key".into()),
        username: None,
        password: None,
        site: "default".into(),
        console_id: None,
        listen: "127.0.0.1:0".parse().unwrap(),
        poll_interval: std::time::Duration::from_secs(60),
        tls_insecure: true,
        http_timeout: std::time::Duration::from_secs(5),
        max_concurrency: 2,
    };

    let client = ReadOnlyClient::new(&cfg).unwrap();
    let info = client.info().await.unwrap();
    assert_eq!(info.application_version.as_deref(), Some("10.4.57"));
}
