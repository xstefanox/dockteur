#[cfg(test)]
#[path = "./ui_test.rs"]
mod ui_test;

use std::fmt::{Display, Formatter};
use ConfigurationError::{InvalidPort, InvalidTimeout};
use State::{Healthy, Unhealthy};
use crate::{ConfigurationError, State};

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Healthy => write!(f, "healthy"),
            Unhealthy => write!(f, "unhealthy"),
        }
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidPort(port) => write!(f, "invalid port '{}'", port),
            InvalidTimeout(timeout) => write!(f, "invalid timeout '{}'", timeout),
        }
    }
}

pub trait ExitCode {
    fn to_exit_code(&self) -> i32;
}

impl ExitCode for Result<State, ConfigurationError> {
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
