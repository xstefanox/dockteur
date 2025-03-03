use std::collections::HashMap;
use std::env;
use std::time::Duration;
use crate::configuration;
use crate::configuration::Configuration;
use crate::health_checker::http::Http;

pub(crate) mod http;

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Healthy,
    Unhealthy(Reason),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Reason {
    TimedOut(Duration),
    UnexpectedStatusCode(u16, String),
}

#[derive(Debug, PartialEq)]
struct NetworkError {
    message: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct HeathcheckFailure {
    message: String,
}

trait HealthCheck {

    async fn get_health(&self, configuration: &Configuration) -> Result<State, NetworkError>;
}

pub async fn run_health_check() -> Result<State, HeathcheckFailure> {
    let vars: HashMap<String, String> = env::vars().collect();

    let configuration = configuration::load_configuration_from(vars)
        .map_err(|err| {
            HeathcheckFailure {
                message: err.to_string(),
            }
        })?;

    Http.get_health(&configuration).await.map_err(|err| {
        HeathcheckFailure {
            message: err.message,
        }
    })
}
