extern crate core;

use env_logger::Target;
use log::LevelFilter;

use crate::health_checker::run_health_check;
use crate::system::ExitCode;

mod configuration;

mod health_checker;

mod system;

#[cfg(test)]
mod test_logger;

#[cfg(test)]
mod test_macros;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .format_target(false)
        .init();

    let result = run_health_check().await;

    std::process::exit(result.to_exit_code());
}
