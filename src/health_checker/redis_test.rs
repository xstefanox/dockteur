use crate::health_checker::Reason::Timeout;
use crate::health_checker::State::Healthy;
use crate::health_checker::State::Unhealthy;
use assert2::{check, assert};
use std::net::TcpListener;
use std::time::Duration;
use testcontainers_modules::testcontainers::core::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::REDIS_PORT;
use crate::configuration::fixtures::{a_redis_configuration, a_redis_configuration_with_timeout};
use crate::health_checker::redis::Redis;
use crate::health_checker::HealthCheck;
use crate::health_checker::toxiproxy::{ToxiProxyContainer, PROXY_PORT};

#[tokio::test]
async fn a_healthy_redis_should_be_reported() {
    let redis_container = testcontainers_modules::redis::Redis::default()
        .start()
        .await
        .unwrap();

    let port = redis_container.get_host_port_ipv4(REDIS_PORT).await.unwrap();
    let configuration = a_redis_configuration(port);

    let result = Redis.get_health(&configuration).await;

    assert!(let Ok(state) = result);
    check!(state == Healthy);
}

#[tokio::test]
async fn unreachable_redis_should_be_reported_as_error() {
    let unused_port = TcpListener::bind("localhost:0").unwrap()
        .local_addr().unwrap()
        .port();
    let configuration = a_redis_configuration(unused_port);

    let result = Redis.get_health(&configuration).await;

    assert!(let Err(error) = result);
    check!(error.message.starts_with("network error"));
}

#[tokio::test]
async fn redis_responding_slowly_should_be_reported_as_unhealthy() {
    let network = "dockteur-redis-test-network";
    let redis_name = "redis-service";

    let _redis = testcontainers_modules::redis::Redis::default()
        .with_network(network)
        .with_container_name(redis_name)
        .start()
        .await
        .unwrap();

    let toxiproxy = ToxiProxyContainer
        .with_network(network)
        .start()
        .await
        .unwrap();

    let client = ToxiProxyContainer::create_client(&toxiproxy).await;
    client.create_proxy("redis-server", PROXY_PORT, redis_name, REDIS_PORT).await;
    client.add_latency("redis-server", 2_000).await;

    let toxiproxy_port = toxiproxy.get_host_port_ipv4(PROXY_PORT).await.unwrap();
    let configuration = a_redis_configuration_with_timeout(toxiproxy_port, 1);

    let result = Redis.get_health(&configuration).await;

    assert!(let Ok(state) = result);
    check!(state == Unhealthy(Timeout(Duration::from_millis(1))));
}
