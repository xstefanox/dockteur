use std::collections::HashMap;
use std::env;
use log::{debug, error, info};
use reqwest::Client;
use url::Url;
use std::time::Duration;
use crate::configuration;
use crate::configuration::Configuration;
use crate::health_checker::Reason::{UnexpectedStatusCode, TimedOut};

#[cfg(test)]
#[path = "./health_checker_test.rs"]
mod test;

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

async fn get_health(configuration: &Configuration) -> Result<State, NetworkError> {
    let mut url = Url::parse("http://localhost").unwrap();
    url.set_port(Some(configuration.port.into())).unwrap();
    let x: String = configuration.path.clone().into();
    url.set_path(&x);

    let client = Client::builder()
        .timeout(configuration.timeout.into())
        .build()
        .unwrap();

    let response = client
        .request(configuration.method.clone().into(), url.as_ref())
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
                Ok(State::Unhealthy(UnexpectedStatusCode(value.status().as_u16(), reason)))
            }
        }
        Err(e) => {
            if e.is_timeout() {
                Ok(State::Unhealthy(TimedOut(configuration.timeout.into())))
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

    let configuration = configuration::load_configuration_from(vars)
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
