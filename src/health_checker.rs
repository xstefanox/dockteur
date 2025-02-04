use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use http::Method;
use log::{debug, error, info};
use reqwest::Client;
use url::Url;
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
    use http::Method;

    pub(super) const METHOD: Method = Method::GET;
    pub(super) const PORT: u16 = 80;
    pub(super) const PATH: &str = "/";
    pub(super) const TIMEOUT: Duration = Duration::from_millis(500);
    pub(super) const STATUS_CODE: u16 = 200;
}

#[derive(Debug)]
struct Configuration {
    method: Method,
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
    Method(String),
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

#[macro_export]
macro_rules! env {
    ( $x:expr ) => {
        format!("DOCKTEUR_{}", $x).as_str()
    };
}

fn sanitize(value: &str) -> Option<String> {
    Some(value.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn load_method_from(vars: &HashMap<String, String>) -> Result<Method, InvalidConfiguration> {
    match vars.get(env!("METHOD")) {
        None => Ok(default::METHOD),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::METHOD),
                Some(value) => {
                    Method::from_str(value.as_str())
                        .map_err(|_| InvalidConfiguration::Method(value))
                },
            }
        }
    }
}

fn load_port_from(vars: &HashMap<String, String>) -> Result<u16, InvalidConfiguration> {
    let env_var = vars.get(env!("PORT"))
        .or(vars.get("PORT"));

    match env_var {
        None => Ok(default::PORT),
        Some(value) => {
            match sanitize(value) {
                None => Ok(default::PORT),
                Some(value) => match value.parse::<u16>() {
                    Ok(number) => match number {
                        0 => Err(InvalidConfiguration::Port(value.clone())),
                        _ => Ok(number),
                    },
                    Err(_) => Err(InvalidConfiguration::Port(value.clone())),
                }
            }
        }
    }
}

fn load_path_from(vars: &HashMap<String, String>) -> Result<String, InvalidConfiguration> {
    match vars.get(env!("PATH")) {
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
    match vars.get(env!("TIMEOUT_MILLIS")) {
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
    match vars.get(env!("STATUS_CODE")) {
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

async fn get_health(configuration: &Configuration) -> Result<State, NetworkError> {
    let mut url = Url::parse("http://localhost").unwrap();
    url.set_port(Some(configuration.port)).unwrap();
    url.set_path(&configuration.path);

    let client = Client::builder()
        .timeout(configuration.timeout)
        .build()
        .unwrap();

    let response = client
        .request(configuration.method.clone(), url.as_ref())
        .send()
        .await;

    debug!("received result from {}: {:?}", url, response);

    let result: Result<State, NetworkError> = match response {
        Ok(value) => {
            if value.status() == configuration.status_code {
                Ok(State::Healthy)
            } else {
                let reason = value
                    .status()
                    .canonical_reason()
                    .unwrap_or("")
                    .to_string();
                Ok(State::Unhealthy(StatusCode(value.status().as_u16(), reason)))
            }
        }
        Err(e) => {
            if e.is_timeout() {
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

pub async fn run_health_check() -> Result<State, HeathcheckFailure> {
    let vars: HashMap<String, String> = env::vars().collect();

    let configuration = load_configuration_from(vars)
        .map_err(|err| {
            HeathcheckFailure {
                message: err.to_string(),
            }
        })?;

    get_health(&configuration).await.map_err(|err| {
        HeathcheckFailure {
            message: err.message,
        }
    })
}
