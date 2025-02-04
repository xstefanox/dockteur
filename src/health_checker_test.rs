use crate::health_checker::Reason::{StatusCode, Timeout};
use crate::health_checker::State::Healthy;
use crate::health_checker::State::Unhealthy;
use crate::health_checker::{default, get_health, Configuration};
use assert2::{check, let_assert};
use rand::Rng;
use std::net::TcpListener;
use std::time::Duration;
use http::Method;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn a_healthy_service_should_be_reported() {
    let mock_server = MockServer::start().await;
    let status_code = a_status_code();
    let configuration = client_configuration_with_status_code(mock_server.address().port(), status_code);
    mock_server_health(&mock_server, status_code).await;

    let result = get_health(&configuration).await;

    let_assert!(Ok(state) = result);
    check!(state == Healthy);
}

#[tokio::test]
async fn an_unhealthy_service_should_be_reported() {
    let mock_server = MockServer::start().await;
    let configuration = client_configuration(mock_server.address().port());
    mock_server_health(&mock_server, 500).await;

    let result = get_health(&configuration).await;

    let_assert!(Ok(state) = result);
    check!(state == Unhealthy(StatusCode(500, "Internal Server Error".to_string())));
}

#[tokio::test]
async fn service_responding_slowly_should_be_reported_as_unhealthy() {
    let mock_server = MockServer::start().await;
    let configuration = client_configuration_with_timeout(mock_server.address().port(), 1);
    mock_server_health_with_delay(&mock_server, 500, 1_000).await;

    let result = get_health(&configuration).await;

    let_assert!(Ok(state) = result);
    check!(state == Unhealthy(Timeout(Duration::from_millis(1))));
}

#[tokio::test]
async fn on_network_error_the_service_should_be_reported_as_error() {
    let unused_port = TcpListener::bind("localhost:0").unwrap()
        .local_addr().unwrap()
        .port();
    let configuration = client_configuration(unused_port);

    let result = get_health(&configuration).await;

    let_assert!(Err(error) = result);
    check!(error.message.starts_with("network error"));
}

async fn mock_server_health(mock_server: &MockServer, status_code: u16) {
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(status_code))
        .mount(mock_server)
        .await
}

async fn mock_server_health_with_delay(mock_server: &MockServer, status_code: u16, delay_millis: u64) {
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(status_code).set_delay(Duration::from_millis(delay_millis)))
        .mount(mock_server)
        .await
}

fn client_configuration(port: u16) -> Configuration {
    client_configuration_with_timeout(port, default::TIMEOUT.as_millis() as u64)
}

fn client_configuration_with_status_code(port: u16, status_code: u16) -> Configuration {
    Configuration {
        method: Method::GET,
        port,
        path: "/health".to_string(),
        timeout: default::TIMEOUT,
        status_code,
    }
}


fn client_configuration_with_timeout(port: u16, timeout: u64) -> Configuration {
    Configuration {
        method: Method::GET,
        port,
        path: "/health".to_string(),
        timeout: Duration::from_millis(timeout),
        status_code: 200,
    }
}

fn a_status_code() -> u16 {
    let mut rng = rand::rng();
    rng.random_range(200..226)
}
