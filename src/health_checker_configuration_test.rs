use std::time::Duration;
use assertables::{assert_err_eq, assert_ok};
use crate::map;
use crate::health_checker::InvalidConfiguration;

#[test]
fn service_method_should_be_read_from_environment_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => "HEAD",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.method, "HEAD");
}

#[test]
fn service_method_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {});

    let configuration = assert_ok!(result);
    assert_eq!(configuration.method, "GET");
}

#[test]
fn empty_service_method_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => "",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.method, "GET");
}

#[test]
fn blank_service_method_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => " ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.method, "GET");
}

#[test]
fn service_method_should_be_trimmed() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => " POST ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.method, "POST");
}

#[test]
fn expected_status_code_should_be_read_from_environment_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "201",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.status_code, 201);
}

#[test]
fn expected_status_code_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {});

    let configuration = assert_ok!(result);
    assert_eq!(configuration.status_code, 200);
}

#[test]
fn malformed_status_code_should_not_be_accepted() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "MALFORMED",
    });

    assert_err_eq!(result, InvalidConfiguration::StatusCode("MALFORMED".to_string()));
}

#[test]
fn empty_status_code_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.status_code, 200);
}

#[test]
fn blank_status_code_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => " ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.status_code, 200);
}

#[test]
fn service_port_should_be_read_from_environment_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PORT" => "8080",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.port, 8080);
}

#[test]
fn service_port_should_be_read_from_common_environment_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "PORT" => "8080",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.port, 8080);
}

#[test]
fn port_specific_variable_should_have_precedence_on_common_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PORT" => "8081",
        "PORT" => "8080",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.port, 8081);
}

#[test]
fn service_port_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {});

    let configuration = assert_ok!(result);
    assert_eq!(configuration.port, 80);
}

#[test]
fn malformed_service_port_should_not_be_accepted() {
    let result = crate::health_checker::load_configuration_from(map! {
        "PORT" => "MALFORMED",
    });

    assert_err_eq!(result, InvalidConfiguration::Port("MALFORMED".to_string()));
}

#[test]
fn port_non_representable_in_16_bits_should_not_be_accepted() {
    let result = crate::health_checker::load_configuration_from(map! {
        "PORT" => "65536",
    });

    assert_err_eq!(result, InvalidConfiguration::Port("65536".to_string()));
}

#[test]
fn port_0_should_not_be_accepted() {
    let result = crate::health_checker::load_configuration_from(map! {
        "PORT" => "0",
    });

    assert_err_eq!(result, InvalidConfiguration::Port("0".to_string()));
}

#[test]
fn empty_service_port_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PORT" => "",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.port, 80);
}

#[test]
fn blank_service_port_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PORT" => " ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.port, 80);
}

#[test]
fn service_path_should_be_read_from_environment_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PATH" => "/this/is/the/path",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.path, "/this/is/the/path");
}

#[test]
fn service_path_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {});

    let configuration = assert_ok!(result);
    assert_eq!(configuration.path, "/");
}

#[test]
fn empty_service_path_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PATH" => "",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.path, "/");
}

#[test]
fn blank_service_path_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PATH" => " ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.path, "/");
}

#[test]
fn service_path_should_be_trimmed() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_PATH" => " /this/is/the/path ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.path, "/this/is/the/path");
}

#[test]
fn timeout_should_be_read_from_environment_variable() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => "100",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.timeout, Duration::from_millis(100));
}

#[test]
fn timeout_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {});

    let configuration = assert_ok!(result);
    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn malformed_timeout_port_should_not_be_accepted() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => "MALFORMED",
    });

    assert_err_eq!(result, InvalidConfiguration::Timeout("MALFORMED".to_string()));
}

#[test]
fn empty_timeout_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => "",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn blank_timeout_should_fallback_on_default() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => " ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn timeout_should_be_trimmed() {
    let result = crate::health_checker::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => " 100 ",
    });

    let configuration = assert_ok!(result);
    assert_eq!(configuration.timeout, Duration::from_millis(100));
}
