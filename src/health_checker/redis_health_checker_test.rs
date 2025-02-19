use crate::health_checker::redis_health_checker::redis_test::toxiproxy::{get_client, ToxiProxyContainer};
use crate::health_checker::Reason::{Generic, Timeout};
use crate::health_checker::State::{Healthy, Unhealthy};
use crate::health_checker::{
    default, Configuration, HealthChecker, MockPingCommand, NetworkError, PingCommand,
    RedisHealthChecker, TcpPingCommand,
};
use assert2::{check, let_assert};
use log::debug;
use std::net::TcpListener;
use std::time::Duration;
use http::Method;
use testcontainers_modules::redis::{Redis as RedisContainer, REDIS_PORT};
use testcontainers_modules::testcontainers::core::ContainerPort::Tcp;
use testcontainers_modules::testcontainers::core::WaitFor;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::{GenericImage, Image, ImageExt};
use tokio::time::{sleep, sleep_until};
use crate::health_checker::test_fixtures::{client_configuration, client_configuration_with_timeout};

#[cfg(test)]
#[path = "toxiproxy.rs"]
mod toxiproxy;

#[tokio::test]
async fn a_healthy_service_should_be_reported() {
    let redis_container = RedisContainer::default().start().await.unwrap();
    let port = redis_container
        .get_host_port_ipv4(REDIS_PORT)
        .await
        .unwrap();
    let configuration = client_configuration_with_timeout(port, 456);

    let result = RedisHealthChecker {
        ping_command: &TcpPingCommand {} as &dyn PingCommand,
    }
    .check(&configuration).await;

    let_assert!(Ok(state) = result);
    check!(state == Healthy);
}

#[tokio::test]
async fn an_unhealthy_service_should_be_reported() {
    let redis_container = RedisContainer::default().start().await.unwrap();
    let port = redis_container
        .get_host_port_ipv4(REDIS_PORT)
        .await
        .unwrap();
    let configuration = client_configuration_with_timeout(port, 456);
    let mut ping_command = MockPingCommand::new();

    ping_command.expect_run().return_const(false);

    let result = RedisHealthChecker {
        ping_command: &ping_command as &dyn PingCommand,
    }
    .check(&configuration).await;

    let_assert!(Ok(state) = result);
    check!(state == Unhealthy(Generic("TODO".to_string()))); // TODO provide a specific reason
}

#[tokio::test]
async fn service_responding_slowly_should_be_reported_as_unhealthy() {
    let redis_container = RedisContainer::default()
        .start()
        .await
        .unwrap();

    let redis_host = redis_container
        .get_bridge_ip_address()
        .await
        .unwrap()
        .to_string();
    debug!("redis is listening on port {}", redis_host);

    let toxiproxy_container = ToxiProxyContainer::default().start().await.unwrap();
    let toxiproxy_client = get_client(&toxiproxy_container).await;
    toxiproxy_client.set_timeout("redis", 8080, &redis_host, REDIS_PORT, 1_000);

    let exposed_proxy_port = toxiproxy_container.get_host_port_ipv4(8080).await.unwrap();
    debug!("exposed port is {}", exposed_proxy_port);

    let configuration = client_configuration_with_timeout(exposed_proxy_port, 1);

    let result = RedisHealthChecker {
        ping_command: &TcpPingCommand {} as &dyn PingCommand,
    }
    .check(&configuration).await;

    let_assert!(Err(e) = result);
    let_assert!(NetworkError { message } = e);
}

#[tokio::test]
async fn on_network_error_the_service_should_be_reported_as_error() {
    let unused_port = TcpListener::bind("localhost:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let configuration = client_configuration(unused_port);

    let result = RedisHealthChecker {
        ping_command: &TcpPingCommand {} as &dyn PingCommand,
    }
    .check(&configuration).await;

    let_assert!(Err(error) = result);
    check!(error.message.starts_with("network error"));
}
