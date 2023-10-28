use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::time::Duration;

use log::debug;
use ureq::OrAnyStatus;

use ConfigurationError::{InvalidPort, InvalidTimeout};

#[cfg(test)]
#[path = "./health_checker_test.rs"]
mod test;

mod default {
    use std::time::Duration;

    pub(super) const METHOD: &str = "GET";
    pub(super) const PORT: u16 = 80;
    pub(super) const PATH: &str = "/";
    pub(super) const TIMEOUT: Duration = Duration::from_millis(500);
}

#[derive(Debug)]
struct Configuration {
    method: String,
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

fn sanitize(value: &String) -> Option<String> {
    return Some(value.trim().to_string())
        .filter(|s| !s.is_empty());
}

fn load_method_from(vars: &HashMap<String, String>) -> Result<String, ConfigurationError> {
    return match vars.get("HEALTHCHECK_METHOD") {
        None => Ok(default::METHOD.into()),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::METHOD.to_string()),
                Some(value) => Ok(value.clone()),
            }
        }
    };
}

fn load_port_from(vars: &HashMap<String, String>) -> Result<u16, ConfigurationError> {
    let env_var = vars.get("HEALTHCHECK_PORT")
        .or(vars.get("PORT"));

    return match env_var {
        None => Ok(default::PORT),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::PORT),
                Some(value) => match value.parse::<u16>() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(InvalidPort(value.clone())),
                }
            }
        }
    };
}

fn load_path_from(vars: &HashMap<String, String>) -> Result<String, ConfigurationError> {
    return match vars.get("HEALTHCHECK_PATH") {
        None => Ok(default::PATH.to_string()),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::PATH.to_string()),
                Some(value) => Ok(value.clone())
            }
        }
    };
}

fn load_timeout_from(vars: &HashMap<String, String>) -> Result<Duration, ConfigurationError> {
    return match vars.get("HEALTHCHECK_TIMEOUT_MILLIS") {
        None => Ok(default::TIMEOUT),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::TIMEOUT),
                Some(value) => match value.parse::<u64>() {
                    Ok(value) => Ok(Duration::from_millis(value)),
                    Err(_) => Err(InvalidTimeout(value)),
                }
            }
        }
    };
}

fn load_configuration_from(vars: HashMap<String, String>) -> Result<Configuration, ConfigurationError> {
    let method = load_method_from(&vars)?;
    let port = load_port_from(&vars)?;
    let path = load_path_from(&vars)?;
    let timeout = load_timeout_from(&vars)?;
    return Ok(Configuration { method, port, path, timeout });
}

fn get_health(configuration: &Configuration) -> State {
    let url = format!("http://localhost:{}{}", configuration.port, configuration.path);
    let agent = ureq::AgentBuilder::new()
        .timeout_read(configuration.timeout)
        .build();
    let response_status = agent
        .request(configuration.method.borrow(), &*url)
        .call()
        .or_any_status()
        .map(|response| response.status());

    debug!("received status code {:?} from {}", response_status, url);

    return match response_status {
        Ok(value) => match value {
            200 => State::Healthy,
            _ => State::Unhealthy,
        }
        Err(_) => State::Unhealthy,
    };
}

pub fn run_health_check() -> Result<State, ConfigurationError> {
    let vars: HashMap<String, String> = env::vars().collect();
    return load_configuration_from(vars)
        .map(|configuration| get_health(&configuration));
}
