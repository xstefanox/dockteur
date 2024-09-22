use crate::ExitCode;
use crate::health_checker::InvalidConfiguration;
use crate::State::{Healthy, Unhealthy};

#[test]
fn healthy_state_should_be_converted_to_process_exit_status() {
    let status = Ok(Healthy).to_exit_code();

    assert_eq!(0, status);
}

#[test]
fn unhealthy_state_should_be_converted_to_process_exit_status() {
    let status = Ok(Unhealthy).to_exit_code();

    assert_eq!(1, status);
}

#[test]
fn invalid_port_message() {
    let err = InvalidConfiguration::Port(String::from("MALFORMED"));

    let result = format!("{}", err);

    assert_eq!("invalid port 'MALFORMED'", result)
}

#[test]
fn invalid_timeout_message() {
    let err = InvalidConfiguration::Timeout(String::from("MALFORMED"));

    let result = format!("{}", err);

    assert_eq!("invalid timeout 'MALFORMED'", result)
}

#[test]
fn invalid_status_code_message() {
    let err = InvalidConfiguration::StatusCode(String::from("MALFORMED"));

    let result = format!("{}", err);

    assert_eq!("invalid status code 'MALFORMED'", result)
}
