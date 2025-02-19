use crate::health_checker::Reason::{Generic, Timeout};
use crate::health_checker::{Configuration, HealthChecker, NetworkError, State};
use log::{debug, error, info};
use reqwest::Client;
use url::Url;

#[cfg(test)]
#[path = "./redis_health_checker_test.rs"]
mod redis_test;

pub struct Http;

impl HealthChecker for Http {
    // TODO receive only the required configuration
    async fn check(&self, configuration: &Configuration) -> Result<State, NetworkError> {
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
                    // let reason = value.status().canonical_reason().unwrap_or("").to_string();

                    Ok(State::Unhealthy(Generic(value.status().to_string())))
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    Ok(State::Unhealthy(Timeout(configuration.timeout)))
                } else {
                    Err(NetworkError {
                        message: format!("network error: {}", e),
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
}
