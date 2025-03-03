use http::Method;
use std::time::Duration;
use std::collections::HashMap;
use std::str::FromStr;

#[cfg(test)]
#[path = "./configuration_test.rs"]
mod test;

// REFACTOR replace with an implementation of the Default trait
pub(crate) mod default {
    use http::Method;
    use std::time::Duration;


    pub(super) const METHOD: Method = Method::GET;
    pub(super) const PORT: u16 = 80;
    pub(super) const PATH: &str = "/";
    // REFACTOR using the Default trait should allow restricting the visibility
    pub(crate) const TIMEOUT: Duration = Duration::from_millis(500);
    pub(super) const STATUS_CODE: u16 = 200;
}

#[derive(Debug)]
pub(crate) struct Configuration {
    pub(crate) method: Method,
    pub(crate) port: u16,
    pub(crate) path: String,
    pub(crate) timeout: Duration,
    pub(crate) status_code: u16,
}

#[derive(Debug, PartialEq)]
pub(crate) enum InvalidConfiguration {
    Port(String),
    Timeout(String),
    StatusCode(String),
    Method(String),
}

#[macro_export]
macro_rules! env {
    ( $x:expr ) => {
        format!("DOCKTEUR_{}", $x).as_str()
    };
}

pub fn sanitize(value: &str) -> Option<String> {
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

pub(crate) fn load_configuration_from(vars: HashMap<String, String>) -> Result<Configuration, InvalidConfiguration> {
    let method = load_method_from(&vars)?;
    let port = load_port_from(&vars)?;
    let path = load_path_from(&vars)?;
    let timeout = load_timeout_from(&vars)?;
    let status_code = load_status_code_from(&vars)?;
    Ok(Configuration { method, port, path, timeout, status_code })
}