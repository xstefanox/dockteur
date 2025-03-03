use crate::configuration::{Configuration, Port, StatusCode, Timeout};
use std::time::Duration;

pub(crate) fn a_configuration(port: u16) -> Configuration {
    Configuration {
        port: Port(port),
        ..Default::default()
    }
}

pub(crate) fn a_configuration_with_status_code(port: u16, status_code: u16) -> Configuration {
    Configuration {
        port: Port(port),
        status_code: StatusCode(status_code),
        ..Default::default()
    }
}

pub(crate) fn a_configuration_with_timeout(port: u16, timeout: u64) -> Configuration {
    Configuration {
        port: Port(port),
        timeout: Timeout(Duration::from_millis(timeout)),
        ..Default::default()
    }
}
