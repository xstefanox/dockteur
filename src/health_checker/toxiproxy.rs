use default::ADMIN_PORT;
use assert2::check;
use http::StatusCode;
use serde::Serialize;
use testcontainers_modules::testcontainers::core::wait::LogWaitStrategy;
use testcontainers_modules::testcontainers::core::{ContainerPort, WaitFor};
use testcontainers_modules::testcontainers::{ContainerAsync, Image};
use url::Url;
use ContainerPort::Tcp;

mod default {
    pub const ADMIN_PORT: u16 = 8474;
}

pub const PROXY_PORT: u16 = 8080;

#[derive(Default)]
pub struct ToxiProxyContainer;

impl Image for ToxiProxyContainer {
    fn name(&self) -> &str {
        "ghcr.io/shopify/toxiproxy"
    }

    fn tag(&self) -> &str {
        "2.12.0"
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::log(LogWaitStrategy::stdout(
            "Starting Toxiproxy HTTP server",
        ))]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[
            Tcp(ADMIN_PORT),
            Tcp(PROXY_PORT),
        ]
    }
}

impl ToxiProxyContainer {
    pub async fn create_client(container: &ContainerAsync<ToxiProxyContainer>) -> ToxiproxyClient {
        let host = container.get_host().await.unwrap().to_string();
        let admin_port = container.get_host_port_ipv4(ADMIN_PORT).await.unwrap();

        ToxiproxyClient::new(&host, admin_port)
    }
}

#[derive(Serialize)]
struct LatencyToxicAttributes {
    latency: u64,
    jitter: u64,
}

#[derive(Serialize)]
struct ToxicCreationRequest {
    r#type: String,
    attributes: LatencyToxicAttributes,
}

#[derive(Serialize)]
struct ProxyCreationRequest {
    name: String,
    listen: String,
    upstream: String,
    enabled: bool,
}

impl ProxyCreationRequest {
    pub fn new(name: &str, proxy_port: u16, upstream_host: &str, upstream_port: u16) -> Self {
        ProxyCreationRequest {
            name: name.into(),
            listen: format!("0.0.0.0:{}", proxy_port),
            upstream: format!("{}:{}", upstream_host, upstream_port),
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

    pub async fn create_proxy(
        &self,
        name: &str,
        proxy_port: u16,
        upstream_host: &str,
        upstream_port: u16,
    ) {
        let url = self.build_url("/proxies");
        let client = reqwest::Client::new();

        let status = client
            .post(url)
            .json(&ProxyCreationRequest::new(
                name,
                proxy_port,
                upstream_host,
                upstream_port,
            ))
            .send()
            .await
            .expect("unexpected proxy creation failure")
            .status();

        check!(status == StatusCode::CREATED);
    }

    pub async fn add_latency(&self, proxy_name: &str, latency_ms: u64) {
        let url = self.build_url(&format!("/proxies/{}/toxics", proxy_name));
        let client = reqwest::Client::new();

        let status = client
            .post(url)
            .json(&ToxicCreationRequest {
                r#type: "latency".into(),
                attributes: LatencyToxicAttributes {
                    latency: latency_ms,
                    jitter: 0,
                },
            })
            .send()
            .await
            .expect("unexpected toxic creation failure")
            .status();

        check!(status == StatusCode::OK);
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
