use crate::State::{Healthy, Unhealthy};
use crate::ExitCode;
use crate::health_checker::ConfigurationError::{InvalidPort, InvalidStatusCode, InvalidTimeout};

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
    let err = InvalidPort(String::from("MALFORMED"));

    let result = format!("{}", err);

    assert_eq!("invalid port 'MALFORMED'", result)
}

#[test]
fn invalid_timeout_message() {
    let err = InvalidTimeout(String::from("MALFORMED"));

    let result = format!("{}", err);

    assert_eq!("invalid timeout 'MALFORMED'", result)
}

#[test]
fn invalid_status_code_message() {
    let err = InvalidStatusCode(String::from("MALFORMED"));

    let result = format!("{}", err);

    assert_eq!("invalid status code 'MALFORMED'", result)
}
