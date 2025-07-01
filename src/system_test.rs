use rstest::rstest;
use crate::ExitCode;
use crate::health_checker::Reason;
use crate::health_checker::Reason::{TimedOut, UnexpectedStatusCode};
use crate::health_checker::State::{Healthy, Unhealthy};
use std::time::Duration;
use crate::configuration::InvalidConfiguration;

#[test]
fn healthy_state_should_be_converted_to_process_exit_status() {
    let status = Ok(Healthy).to_exit_code();

    assert_eq!(0, status);
}

#[rstest]
#[case::timeout(TimedOut(Duration::default()))]
#[case::status_code(UnexpectedStatusCode(500, "Internal server error".to_string()))]
fn unhealthy_state_should_be_converted_to_process_exit_status(#[case] reason: Reason) {
    let status = Ok(Unhealthy(reason)).to_exit_code();

    assert_eq!(1, status);
}

#[test]
fn invalid_port_message() {
    let err = InvalidConfiguration::Port(String::from("MALFORMED"));

    let result = format!("{err}");

    assert_eq!("invalid port 'MALFORMED'", result)
}

#[test]
fn invalid_timeout_message() {
    let err = InvalidConfiguration::Timeout(String::from("MALFORMED"));

    let result = format!("{err}");

    assert_eq!("invalid timeout 'MALFORMED'", result)
}

#[test]
fn invalid_status_code_message() {
    let err = InvalidConfiguration::StatusCode(String::from("MALFORMED"));

    let result = format!("{err}");

    assert_eq!("invalid status code 'MALFORMED'", result)
}

#[test]
fn invalid_method_message() {
    let err = InvalidConfiguration::Method(String::from("MALFORMED"));

    let result = format!("{err}");

    assert_eq!("invalid method 'MALFORMED'", result)
}
