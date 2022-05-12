extern crate core;

use env_logger::Target;
use log::{error, info, LevelFilter};
use crate::health_checker::{ConfigurationError, run_health_check, State, ui::ExitCode};

mod health_checker;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();

    let result = run_health_check();

    match &result {
        Ok(state) => info!("state {}", state),
        Err(error) => error!("{}", error),
    };

    std::process::exit(result.to_exit_code());
}
