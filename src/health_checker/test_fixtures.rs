use crate::health_checker::{default, Configuration};
use http::Method;
use std::time::Duration;

pub(super) fn client_configuration(port: u16) -> Configuration {
    client_configuration_with_timeout(port, default::TIMEOUT.as_millis() as u64)
}

pub(super) fn client_configuration_with_status_code(port: u16, status_code: u16) -> Configuration {
    Configuration {
        method: Method::GET,
        port,
        path: "/health".to_string(),
        timeout: default::TIMEOUT,
        status_code,
    }
}

pub(super) fn client_configuration_with_timeout(port: u16, timeout: u64) -> Configuration {
    Configuration {
        method: Method::GET,
        port,
        path: "/health".to_string(),
        timeout: Duration::from_millis(timeout),
        status_code: 200,
    }
}
