use std::time::Duration;
use crate::health_checker::http::test::toxiproxy::default::ADMIN_PORT;
use assert2::check;
use http::StatusCode;
use serde::Serialize;
use testcontainers_modules::testcontainers::core::wait::LogWaitStrategy;
use testcontainers_modules::testcontainers::core::{ContainerPort, WaitFor};
use testcontainers_modules::testcontainers::{ContainerAsync, Image, TestcontainersError};
use url::Url;
use ContainerPort::Tcp;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

mod default {
    pub const ADMIN_PORT: u16 = 8474;
}

#[derive(Default)]
pub struct ToxiProxyContainer;

impl ToxiProxyContainer {

    pub(crate) async fn with(proxy: Proxy) -> Result<ContainerAsync<ToxiProxyContainer>, TestcontainersError> {
        Self::default().start().await
    }
}

pub(crate) struct Upstream {
    pub(crate) host: String,
    pub(crate) port: u16,
}

pub(crate) struct Proxy {
    pub(crate) name: String,
    pub(crate) port: u16,
    pub(crate) upstream: Upstream,
    pub(crate) latency: Duration,
}

impl Image for ToxiProxyContainer {
    fn name(&self) -> &str {
        "ghcr.io/shopify/toxiproxy"
    }

    fn tag(&self) -> &str {
        "2.11.0"
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::log(LogWaitStrategy::stdout(
            "Starting Toxiproxy HTTP server",
        ))]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[
            Tcp(ADMIN_PORT),
            Tcp(8080), // TODO this port should be randomized
        ]
    }
}

pub async fn get_client(container: &ContainerAsync<ToxiProxyContainer>) -> ToxiproxyClient {
    let host = container.get_host().await.unwrap().to_string();
    // ContainerPort();
    let admin_port = container.get_host_port_ipv4(ADMIN_PORT).await.unwrap();

    ToxiproxyClient::new(&host, admin_port)
}

#[derive(Serialize)]
struct TimeoutToxicAttributes {
    timeout: u16,
}

#[derive(Serialize)]
struct ToxicCreationRequest {
    r#type: String,
    attributes: TimeoutToxicAttributes,
}

#[derive(Serialize)]
struct ProxyCreationRequest {
    name: String,
    listen: String,
    upstream: String,
    enabled: bool,
}

impl ProxyCreationRequest {
    pub fn new(name: &str, proxy_port: u16, upstream_address: &str, upstream_port: u16) -> Self {
        ProxyCreationRequest {
            name: name.into(),
            listen: format!("0.0.0.0:{}", proxy_port).into(),
            upstream: format!(
                "{host}:{port}",
                host = upstream_address,
                port = upstream_port,
            )
            .into(),
            enabled: true,
        }
    }
}

pub struct ToxiproxyClient {
    host: String,
    port: u16,
}

impl ToxiproxyClient {
    pub fn new(host: &str, port: u16) -> Self {
        ToxiproxyClient {
            host: host.into(),
            port,
        }
    }

    pub async fn set_timeout(
        &self,
        name: &str,
        proxy_port: u16,
        upstream_host: &str,
        upstream_port: u16,
        timeout: u16,
    ) {
        let url = self.build_url("/proxies");

        let client = reqwest::Client::new();

        let proxy_creation_response_status = client.post(url)
            .json(&ProxyCreationRequest::new(
                name,
                proxy_port,
                &upstream_host,
                upstream_port,
            ))
            .send()
            .await
            .expect("unexpected proxy creation failure")
            .status();

        check!(proxy_creation_response_status == StatusCode::CREATED);

        let url = self.build_url(&format!("/proxies/{name}/toxics", name = name));

        let toxic_creation_response_status = client.post(url)
            .json(&ToxicCreationRequest {
                r#type: "timeout".into(),
                attributes: TimeoutToxicAttributes { timeout },
            })
            .send()
            .await
            .expect("unexpected toxic creation failure")
            .status();

        check!(toxic_creation_response_status == StatusCode::OK);
    }

    fn build_url(&self, path: &str) -> Url {
        let mut url = Url::parse("http://localhost").unwrap();
        url.set_host(Some(&self.host))
            .expect("unexpected Toxiproxy container host");
        url.set_port(Some(self.port))
            .expect("unexpected Toxiproxy container port");
        url.set_path(path);
        url
    }
}
