use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::time::Duration;

use log::{debug, error, info};
use ureq::OrAnyStatus;

use crate::health_checker::Reason::{StatusCode, Timeout};

#[cfg(test)]
#[path = "./health_checker_test.rs"]
mod test;

#[cfg(test)]
#[path = "./health_checker_string_test.rs"]
mod string_test;

#[cfg(test)]
#[path = "./health_checker_configuration_test.rs"]
mod configuration_test;

mod default {
    use std::time::Duration;

    pub(super) const METHOD: &str = "GET";
    pub(super) const PORT: u16 = 80;
    pub(super) const PATH: &str = "/";
    pub(super) const TIMEOUT: Duration = Duration::from_millis(500);
    pub(super) const STATUS_CODE: u16 = 200;
}

#[derive(Debug)]
struct Configuration {
    method: String,
    port: u16,
    path: String,
    timeout: Duration,
    status_code: u16,
}

#[derive(Debug, PartialEq)]
pub enum InvalidConfiguration {
    Port(String),
    Timeout(String),
    StatusCode(String),
}

#[derive(Debug, PartialEq)]
pub enum State {
    Healthy,
    Unhealthy(Reason),
}

#[derive(Debug, PartialEq)]
pub enum Reason {
    Timeout(Duration),
    StatusCode(u16, String),
}

#[derive(Debug, PartialEq)]
pub struct NetworkError {
    message: String,
}

#[derive(Debug, PartialEq)]
pub struct HeathcheckFailure {
    message: String,
}

fn sanitize(value: &str) -> Option<String> {
    Some(value.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn load_method_from(vars: &HashMap<String, String>) -> Result<String, InvalidConfiguration> {
    match vars.get("HEALTHCHECK_METHOD") {
        None => Ok(default::METHOD.into()),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::METHOD.to_string()),
                Some(value) => Ok(value.clone()),
            }
        }
    }
}

fn load_port_from(vars: &HashMap<String, String>) -> Result<u16, InvalidConfiguration> {
    let env_var = vars.get("HEALTHCHECK_PORT")
        .or(vars.get("PORT"));

    match env_var {
        None => Ok(default::PORT),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::PORT),
                Some(value) => match value.parse::<u16>() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(InvalidConfiguration::Port(value.clone())),
                }
            }
        }
    }
}

fn load_path_from(vars: &HashMap<String, String>) -> Result<String, InvalidConfiguration> {
    match vars.get("HEALTHCHECK_PATH") {
        None => Ok(default::PATH.to_string()),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::PATH.to_string()),
                Some(value) => Ok(value.clone()),
            }
        }
    }
}

fn load_timeout_from(vars: &HashMap<String, String>) -> Result<Duration, InvalidConfiguration> {
    match vars.get("HEALTHCHECK_TIMEOUT_MILLIS") {
        None => Ok(default::TIMEOUT),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::TIMEOUT),
                Some(value) => match value.parse::<u64>() {
                    Ok(value) => Ok(Duration::from_millis(value)),
                    Err(_) => Err(InvalidConfiguration::Timeout(value)),
                }
            }
        }
    }
}

fn load_status_code_from(vars: &HashMap<String, String>) -> Result<u16, InvalidConfiguration> {
    match vars.get("HEALTHCHECK_STATUS_CODE") {
        None => Ok(default::STATUS_CODE),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::STATUS_CODE),
                Some(value) => match value.parse::<u16>() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(InvalidConfiguration::StatusCode(value)),
                }
            }
        }
    }
}

fn load_configuration_from(vars: HashMap<String, String>) -> Result<Configuration, InvalidConfiguration> {
    let method = load_method_from(&vars)?;
    let port = load_port_from(&vars)?;
    let path = load_path_from(&vars)?;
    let timeout = load_timeout_from(&vars)?;
    let status_code = load_status_code_from(&vars)?;
    Ok(Configuration { method, port, path, timeout, status_code })
}

fn get_health(configuration: &Configuration) -> Result<State, NetworkError> {
    let url = format!("http://localhost:{}{}", configuration.port, configuration.path);
    let agent = ureq::AgentBuilder::new()
        .timeout_read(configuration.timeout)
        .build();
    let response = agent
        .request(configuration.method.borrow(), &url)
        .call()
        .or_any_status();

    debug!("received result from {}: {:?}", url, response);

    let result = match response {
        Ok(value) => {
            if value.status() == configuration.status_code {
                Ok(State::Healthy)
            } else {
                Ok(State::Unhealthy(StatusCode(value.status(), value.status_text().to_string())))
            }
        }
        Err(e) => {
            if e.to_string().contains("timed out reading response") {
                Ok(State::Unhealthy(Timeout(configuration.timeout)))
            } else {
                Err(NetworkError {
                    message: format!("network error: {}", e)
                })
            }
        }
    };

    match &result {
        Ok(state) => info!("state {}", state),
        Err(failure) => error!("{}", failure.message),
    }

    result
}

pub fn run_health_check() -> Result<State, HeathcheckFailure> {
    let vars: HashMap<String, String> = env::vars().collect();

    let configuration = load_configuration_from(vars)
        .map_err(|err| {
            HeathcheckFailure {
                message: err.to_string(),
            }
        })?;

    get_health(&configuration).map_err(|err| {
        HeathcheckFailure {
            message: err.message,
        }
    })
}
