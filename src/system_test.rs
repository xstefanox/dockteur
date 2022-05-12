use crate::State::{Healthy, Unhealthy};
use crate::ExitCode;

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
