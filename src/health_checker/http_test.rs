use crate::health_checker::Reason::{Timeout, Other};
use crate::health_checker::State::Healthy;
use crate::health_checker::State::Unhealthy;
use assert2::{check, assert};
use rand::RngExt;
use std::net::TcpListener;
use std::time::Duration;
use testcontainers_modules::testcontainers::core::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use crate::configuration::fixtures::{a_configuration, a_configuration_with_status_code, a_configuration_with_timeout};
use crate::health_checker::http::Http;
use crate::health_checker::HealthCheck;
use crate::health_checker::toxiproxy::{ToxiProxyContainer, PROXY_PORT};
use crate::health_checker::whoami::WhoamiContainer;

#[tokio::test]
async fn a_healthy_service_should_be_reported() {
    let mock_server = MockServer::start().await;
    let status_code = a_status_code();
    let configuration = a_configuration_with_status_code(mock_server.address().port(), status_code);
    mock_server_health(&mock_server, status_code).await;

    let result = Http.get_health(&configuration).await;

    assert!(let Ok(state) = result);
    check!(state == Healthy);
}

#[tokio::test]
async fn an_unhealthy_service_should_be_reported() {
    let mock_server = MockServer::start().await;
    let configuration = a_configuration(mock_server.address().port());
    mock_server_health(&mock_server, 500).await;

    let result = Http.get_health(&configuration).await;

    assert!(let Ok(state) = result);
    check!(state == Unhealthy(Other("unexpected status code '500 Internal Server Error'".to_string())));
}

#[tokio::test]
async fn service_responding_slowly_should_be_reported_as_unhealthy() {
    let network = "dockteur-test-network";
    let whoami_name = "whoami-service";

    let _whoami = WhoamiContainer
        .with_network(network)
        .with_container_name(whoami_name)
        .start()
        .await
        .unwrap();

    let toxiproxy = ToxiProxyContainer
        .with_network(network)
        .start()
        .await
        .unwrap();

    let client = ToxiProxyContainer::create_client(&toxiproxy).await;
    client.create_proxy("http-server", PROXY_PORT, whoami_name, 80).await;
    client.add_latency("http-server", 1_000).await;

    let toxiproxy_port = toxiproxy.get_host_port_ipv4(PROXY_PORT).await.unwrap();
    let configuration = a_configuration_with_timeout(toxiproxy_port, 1);

    let result = Http.get_health(&configuration).await;

    assert!(let Ok(state) = result);
    check!(state == Unhealthy(Timeout(Duration::from_millis(1))));
}

#[tokio::test]
async fn on_network_error_the_service_should_be_reported_as_error() {
    let unused_port = TcpListener::bind("localhost:0").unwrap()
        .local_addr().unwrap()
        .port();
    let configuration = a_configuration(unused_port);

    let result = Http.get_health(&configuration).await;

    assert!(let Err(error) = result);
    check!(error.message.starts_with("network error"));
}

async fn mock_server_health(mock_server: &MockServer, status_code: u16) {
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(status_code))
        .mount(mock_server)
        .await
}

fn a_status_code() -> u16 {
    let mut rng = rand::rng();
    rng.random_range(200..226)
}
