use std::collections::HashMap;
use std::env;
use std::time::Duration;
use async_trait::async_trait;
use crate::configuration;
use crate::configuration::Configuration;
use crate::configuration::Protocol;
use crate::health_checker::http::Http;

pub(crate) mod http;

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Healthy,
    Unhealthy(Reason),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Reason {
    Timeout(Duration),
    Other(String),
}

#[derive(Debug, PartialEq)]
struct NetworkError {
    message: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct HeathcheckFailure {
    message: String,
}

#[async_trait]
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

    let checker: Box<dyn HealthCheck> = match configuration.protocol {
        Protocol::Http => Box::new(Http),
    };

    checker.get_health(&configuration).await.map_err(|err| {
        HeathcheckFailure {
            message: err.message,
        }
    })
}
