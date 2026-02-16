use async_trait::async_trait;
use log::{debug, error, info};
use crate::configuration::Configuration;
use crate::health_checker::Reason::{Other, Timeout};
use crate::health_checker::{HealthCheck, NetworkError, State};

#[cfg(test)]
#[path = "./redis_test.rs"]
mod test;

pub(crate) struct Redis;

#[async_trait]
impl HealthCheck for Redis {
    async fn get_health(&self, configuration: &Configuration) -> Result<State, NetworkError> {
        let port: u16 = configuration.port.into();
        let url = format!("redis://localhost:{}", port);

        debug!("connecting to {}", url);

        let client = redis::Client::open(url.as_str())
            .map_err(|e| NetworkError { message: format!("network error: {}", e) })?;

        let timeout: std::time::Duration = configuration.timeout.into();

        let connection = tokio::time::timeout(timeout, client.get_multiplexed_async_connection())
            .await;

        let mut con = match connection {
            Err(_) => {
                let state = State::Unhealthy(Timeout(timeout));
                info!("state {}", state);
                return Ok(state);
            }
            Ok(result) => result.map_err(|e| NetworkError {
                message: format!("network error: {}", e),
            })?,
        };

        let ping_result = tokio::time::timeout(
            timeout,
            redis::cmd("PING").query_async::<String>(&mut con),
        )
        .await;

        let result = match ping_result {
            Err(_) => Ok(State::Unhealthy(Timeout(timeout))),
            Ok(Ok(pong)) if pong == "PONG" => Ok(State::Healthy),
            Ok(Ok(unexpected)) => Ok(State::Unhealthy(Other(format!("unexpected response '{}'", unexpected)))),
            Ok(Err(e)) => Err(NetworkError {
                message: format!("network error: {}", e),
            }),
        };

        match &result {
            Ok(state) => info!("state {}", state),
            Err(failure) => error!("{}", failure.message),
        }

        result
    }
}
