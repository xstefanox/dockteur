extern crate core;

use env_logger::Target;
use log::LevelFilter;

use crate::health_checker::{InvalidConfiguration, run_health_check, State};
use crate::system::ExitCode;

mod health_checker;
mod system;

#[cfg(test)]
mod test_logger;
mod test_macros;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .format_target(false)
        .init();

    let result = run_health_check();

    std::process::exit(result.to_exit_code());
}
