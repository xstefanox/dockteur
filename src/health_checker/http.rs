use crate::configuration::Configuration;
use crate::health_checker::Reason::{TimedOut, UnexpectedStatusCode};
use crate::health_checker::{HealthCheck, NetworkError, State};
use log::{debug, error, info};
use reqwest::Client;
use url::Url;

#[cfg(test)]
#[path = "./http_test.rs"]
mod test;

pub(crate) struct Http;

impl HealthCheck for Http {
    async fn get_health(&self, configuration: &Configuration) -> Result<State, NetworkError> {
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
                    let reason = value.status().canonical_reason().unwrap_or("").to_string();
                    Ok(State::Unhealthy(UnexpectedStatusCode(
                        value.status().as_u16(),
                        reason,
                    )))
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    Ok(State::Unhealthy(TimedOut(configuration.timeout.into())))
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
