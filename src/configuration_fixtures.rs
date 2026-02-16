use crate::configuration::{Configuration, Port, Protocol, StatusCode, Timeout};
use std::num::NonZeroU16;
use std::time::Duration;
use crate::u16nz;

pub(crate) fn a_configuration(port: u16) -> Configuration {
    Configuration {
        port: Port(u16nz!(port)),
        ..Default::default()
    }
}

pub(crate) fn a_configuration_with_status_code(port: u16, status_code: u16) -> Configuration {
    Configuration {
        port: Port(u16nz!(port)),
        status_code: StatusCode(u16nz!(status_code)),
        ..Default::default()
    }
}

pub(crate) fn a_configuration_with_timeout(port: u16, timeout: u64) -> Configuration {
    Configuration {
        port: Port(u16nz!(port)),
        timeout: Timeout(Duration::from_millis(timeout)),
        ..Default::default()
    }
}

pub(crate) fn a_redis_configuration(port: u16) -> Configuration {
    Configuration {
        protocol: Protocol::Redis,
        port: Port(u16nz!(port)),
        ..Default::default()
    }
}

pub(crate) fn a_redis_configuration_with_timeout(port: u16, timeout: u64) -> Configuration {
    Configuration {
        protocol: Protocol::Redis,
        port: Port(u16nz!(port)),
        timeout: Timeout(Duration::from_millis(timeout)),
        ..Default::default()
    }
}
