#[cfg(test)]
#[path = "./health_checker_test.rs"]
mod test;

use std::env;
use std::time::Duration;
use ureq::OrAnyStatus;
use ConfigurationError::{InvalidPort, InvalidTimeout};

mod default {
    use std::time::Duration;

    pub(super) const PORT: u16 = 80;
    pub(super) const PATH: &str = "/";
    pub(super) const TIMEOUT: Duration = Duration::from_millis(500);
}

#[derive(Debug)]
struct Configuration {
    port: u16,
    path: String,
    timeout: Duration,
}

#[derive(Debug, PartialEq)]
pub enum ConfigurationError {
    InvalidPort(String),
    InvalidTimeout(String),
}

#[derive(Debug, PartialEq)]
pub enum State {
    Healthy,
    Unhealthy,
}

fn sanitize(value: String) -> String {
    return value.trim().to_string();
}

fn load_port() -> Result<u16, ConfigurationError> {
    return match env::var("HEALTHCHECK_PORT") {
        Ok(value) => match value.parse::<u16>() {
            Ok(value) => Ok(value),
            Err(_) => Err(InvalidPort(value)),
        }
        Err(_) => Ok(default::PORT),
    };
}

fn load_path() -> Result<String, ConfigurationError> {
    return match env::var("HEALTHCHECK_PATH") {
        Ok(mut value) => {
            value = sanitize(value);
            if value.is_empty() {
                Ok(default::PATH.to_string())
            } else {
                Ok(value)
            }
        }
        Err(_) => Ok(default::PATH.to_string())
    };
}

fn load_timeout() -> Result<Duration, ConfigurationError> {
    match env::var("HEALTHCHECK_TIMEOUT_MILLIS") {
        Ok(mut value) => {
            value = sanitize(value);
            if value.is_empty() {
                Ok(default::TIMEOUT)
            } else {
                match value.parse::<u64>() {
                    Ok(value) => Ok(Duration::from_millis(value)),
                    Err(_) => Err(InvalidTimeout(value)),
                }
            }
        }
        Err(_) => Ok(default::TIMEOUT),
    }
}

fn load_configuration() -> Result<Configuration, ConfigurationError> {
    let port = load_port()?;
    let path = load_path()?;
    let timeout = load_timeout()?;
    return Ok(Configuration { port, path, timeout });
}

fn get_health(configuration: &Configuration) -> State {
    let url = format!("http://localhost:{}{}", configuration.port, configuration.path);
    let agent = ureq::AgentBuilder::new()
        .timeout_read(configuration.timeout)
        .build();
    let response_status = agent.get(&*url)
        .call()
        .or_any_status()
        .map(|response| response.status());

    return match response_status {
        Ok(value) => match value {
            200 => State::Healthy,
            _ => State::Unhealthy,
        }
        Err(_) => State::Unhealthy,
    };
}

pub fn run_health_check() -> Result<State, ConfigurationError> {
    return load_configuration()
        .map(|configuration| get_health(&configuration));
}
