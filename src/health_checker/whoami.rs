use testcontainers_modules::testcontainers::core::wait::LogWaitStrategy;
use testcontainers_modules::testcontainers::core::ContainerPort::Tcp;
use testcontainers_modules::testcontainers::core::{ContainerPort, WaitFor};
use testcontainers_modules::testcontainers::Image;

#[derive(Default)]
pub struct WhoamiContainer;

impl Image for WhoamiContainer {
    fn name(&self) -> &str {
        "traefik/whoami"
    }

    fn tag(&self) -> &str {
        "v1.10"
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::log(LogWaitStrategy::stderr("Starting up on port"))]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[Tcp(80)]
    }
}
