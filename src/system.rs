#[cfg(test)]
#[path = "./system_test.rs"]
mod test;

use std::fmt::{Display, Formatter};
use State::{Healthy, Unhealthy};
use crate::{InvalidConfiguration, State};
use crate::health_checker::HeathcheckFailure;

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Healthy => write!(f, "healthy"),
            Unhealthy => write!(f, "unhealthy"),
        }
    }
}

impl Display for InvalidConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidConfiguration::Port(value) => write!(f, "invalid port '{}'", value),
            InvalidConfiguration::Timeout(value) => write!(f, "invalid timeout '{}'", value),
            InvalidConfiguration::StatusCode(value) => write!(f, "invalid status code '{}'", value),
        }
    }
}

pub trait ExitCode {
    fn to_exit_code(&self) -> i32;
}

impl ExitCode for Result<State, HeathcheckFailure> {
    fn to_exit_code(&self) -> i32 {
        match self {
            Ok(state) => match state {
                Healthy => 0,
                Unhealthy => 1,
            }
            Err(_) => 2,
        }
    }
}
