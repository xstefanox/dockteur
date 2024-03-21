use std::time::Duration;

use tokio;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

use crate::health_checker::{Configuration, ConfigurationError, get_health, load_configuration_from, sanitize};
use crate::health_checker::State::{Healthy, Unhealthy};

#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        {
            let map: std::collections::HashMap<String, String> = vec! [
                $(
                    ($k.to_string(), $v.to_string()),
                )*
            ].iter().cloned().collect();

            map
        }
    };
}

#[test]
fn service_method_should_be_read_from_environment_variable() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_METHOD" => "HEAD",
    }).unwrap();

    assert_eq!(configuration.method, "HEAD");
}

#[test]
fn service_method_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {}).unwrap();

    assert_eq!(configuration.method, "GET");
}

#[test]
fn empty_service_method_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_METHOD" => "",
    }).unwrap();

    assert_eq!(configuration.method, "GET");
}

#[test]
fn blank_service_method_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_METHOD" => " ",
    }).unwrap();

    assert_eq!(configuration.method, "GET");
}

#[test]
fn service_method_should_be_trimmed() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_METHOD" => " POST ",
    }).unwrap();

    assert_eq!(configuration.method, "POST");
}

#[test]
fn service_port_should_be_read_from_environment_variable() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PORT" => "8080",
    }).unwrap();

    assert_eq!(configuration.port, 8080);
}

#[test]
fn service_port_should_be_read_from_common_environment_variable() {
    let configuration = load_configuration_from(map! {
        "PORT" => "8080",
    }).unwrap();

    assert_eq!(configuration.port, 8080);
}

#[test]
fn port_specific_variable_should_have_precedence_on_common_variable() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PORT" => "8081",
        "PORT" => "8080",
    }).unwrap();

    assert_eq!(configuration.port, 8081);
}

#[test]
fn service_port_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {}).unwrap();

    assert_eq!(configuration.port, 80);
}

#[test]
fn malformed_service_port_should_not_be_accepted() {
    let result = load_configuration_from(map! {
        "PORT" => "MALFORMED",
    }).unwrap_err();

    assert_eq!(result, ConfigurationError::InvalidPort("MALFORMED".to_string()));
}

#[test]
fn empty_service_port_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PORT" => "",
    }).unwrap();

    assert_eq!(configuration.port, 80);
}

#[test]
fn blank_service_port_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PORT" => " ",
    }).unwrap();

    assert_eq!(configuration.port, 80);
}

#[test]
fn service_path_should_be_read_from_environment_variable() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PATH" => "/this/is/the/path",
    }).unwrap();

    assert_eq!(configuration.path, "/this/is/the/path");
}

#[test]
fn service_path_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {}).unwrap();

    assert_eq!(configuration.path, "/");
}

#[test]
fn empty_service_path_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PATH" => "",
    }).unwrap();

    assert_eq!(configuration.path, "/");
}

#[test]
fn blank_service_path_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PATH" => " ",
    }).unwrap();

    assert_eq!(configuration.path, "/");
}

#[test]
fn service_path_should_be_trimmed() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_PATH" => " /this/is/the/path ",
    }).unwrap();

    assert_eq!(configuration.path, "/this/is/the/path");
}

#[test]
fn timeout_should_be_read_from_environment_variable() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_TIMEOUT_MILLIS" => "100",
    }).unwrap();

    assert_eq!(configuration.timeout, Duration::from_millis(100));
}

#[test]
fn timeout_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {}).unwrap();

    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn malformed_timeout_port_should_not_be_accepted() {
    let result = load_configuration_from(map! {
        "HEALTHCHECK_TIMEOUT_MILLIS" => "MALFORMED",
    }).unwrap_err();

    assert_eq!(result, ConfigurationError::InvalidTimeout("MALFORMED".to_string()));
}

#[test]
fn empty_timeout_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_TIMEOUT_MILLIS" => "",
    }).unwrap();

    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn blank_timeout_should_fallback_on_default() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_TIMEOUT_MILLIS" => " ",
    }).unwrap();

    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn timeout_should_be_trimmed() {
    let configuration = load_configuration_from(map! {
        "HEALTHCHECK_TIMEOUT_MILLIS" => " 100 ",
    }).unwrap();

    assert_eq!(configuration.timeout, Duration::from_millis(100));
}

#[cfg(not(target_arch = "aarch64"))]
#[tokio::test]
async fn a_healthy_service_should_be_reported() {
    let mock_server = MockServer::start().await;
    let configuration = client_configuration(mock_server.address().port());
    mock_server_health(&mock_server, 200).await;

    let state = get_health(&configuration);

    assert_eq!(Healthy, state);
}

#[tokio::test]
async fn an_unhealthy_service_should_be_reported() {
    let mock_server = MockServer::start().await;
    let configuration = client_configuration(mock_server.address().port());
    mock_server_health(&mock_server, 500).await;

    let state = get_health(&configuration);

    assert_eq!(Unhealthy, state);
}

#[tokio::test]
async fn service_responding_slowly_should_be_reported_as_unhealthy() {
    let mock_server = MockServer::start().await;
    let configuration = client_configuration_with_timeout(mock_server.address().port(), 1);
    mock_server_health_with_delay(&mock_server, 500, 1_000).await;

    let state = get_health(&configuration);

    assert_eq!(Unhealthy, state);
}

#[test]
fn on_network_error_the_service_should_be_reported_as_unhealthy() {
    let configuration = client_configuration(8080);

    let state = get_health(&configuration);

    assert_eq!(Unhealthy, state);
}

#[test]
fn non_empty_string_sanitization() {
    let result = sanitize(&"test".to_string());

    assert_eq!(Some("test".to_string()), result)
}

#[test]
fn empty_string_sanitization() {
    let result = sanitize(&"".to_string());

    assert_eq!(None, result)
}

#[test]
fn blank_string_sanitization() {
    let result = sanitize(&" ".to_string());

    assert_eq!(None, result)
}

async fn mock_server_health(mock_server: &MockServer, status_code: u16) {
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(status_code))
        .mount(&mock_server)
        .await
}

async fn mock_server_health_with_delay(mock_server: &MockServer, status_code: u16, delay_millis: u64) {
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(status_code).set_delay(Duration::from_millis(delay_millis)))
        .mount(&mock_server)
        .await
}

fn client_configuration(port: u16) -> Configuration {
    client_configuration_with_timeout(port, 100)
}

fn client_configuration_with_timeout(port: u16, timeout: u64) -> Configuration {
    Configuration {
        method: "GET".into(),
        port,
        path: "/health".to_string(),
        timeout: Duration::from_millis(timeout),
    }
}
