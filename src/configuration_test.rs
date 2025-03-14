use std::num::NonZeroU16;
use crate::{map, u16nz};
use assert2::{check, let_assert};
use std::time::Duration;
use crate::configuration::{sanitize, InvalidConfiguration, Method, Path, Port, StatusCode, Timeout};

#[test]
fn non_empty_string_sanitization() {
    let result = sanitize("test");

    assert_eq!(Some("test".to_string()), result)
}

#[test]
fn empty_string_sanitization() {
    let result = sanitize("");

    assert_eq!(None, result)
}

#[test]
fn blank_string_sanitization() {
    let result = sanitize(" ");

    assert_eq!(None, result)
}

#[test]
fn test_method_conversion_from_http_method() {
    let http_method = http::Method::GET;

    let result = Method(http_method);

    check!(result == Method(http::Method::GET));
}

#[test]
fn service_method_should_be_read_from_environment_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => "HEAD",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.method == Method(http::Method::HEAD));
}

#[test]
fn service_method_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {});

    let_assert!(Ok(configuration) = result);
    check!(configuration.method == Method(http::Method::GET));
}

#[test]
fn empty_service_method_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => "",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.method == Method(http::Method::GET));
}

#[test]
fn blank_service_method_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => " ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.method == Method(http::Method::GET));
}

#[test]
fn service_method_should_be_trimmed() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => " POST ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.method == Method(http::Method::POST));
}

#[test]
fn malformed_service_method_should_be_reported() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_METHOD" => "DO@SOMETHING",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::Method("DO@SOMETHING".to_string()));
}

#[test]
fn expected_status_code_should_be_read_from_environment_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "201",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.status_code == StatusCode(u16nz!(201)));
}

#[test]
fn expected_status_code_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {});

    let_assert!(Ok(configuration) = result);
    check!(configuration.status_code == StatusCode(u16nz!(200)));
}

#[test]
fn malformed_status_code_should_not_be_accepted() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "MALFORMED",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::StatusCode("MALFORMED".to_string()));
}

#[test]
fn empty_status_code_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.status_code == StatusCode(u16nz!(200)));
}

#[test]
fn blank_status_code_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => " ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.status_code == StatusCode(u16nz!(200)));
}

#[test]
fn status_code_0_should_not_be_accepted() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_STATUS_CODE" => "0",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::StatusCode("0".to_string()));
}

#[test]
fn service_port_should_be_read_from_environment_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PORT" => "8080",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.port == Port(u16nz!(8080)));
}

#[test]
fn service_port_should_be_read_from_common_environment_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "PORT" => "8080",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.port == Port(u16nz!(8080)));
}

#[test]
fn port_specific_variable_should_have_precedence_on_common_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PORT" => "8081",
        "PORT" => "8080",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.port == Port(u16nz!(8081)));
}

#[test]
fn service_port_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {});

    let_assert!(Ok(configuration) = result);
    check!(configuration.port == Port(u16nz!(80)));
}

#[test]
fn malformed_service_port_should_not_be_accepted() {
    let result = crate::configuration::load_configuration_from(map! {
        "PORT" => "MALFORMED",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::Port("MALFORMED".to_string()));
}

#[test]
fn port_non_representable_in_16_bits_should_not_be_accepted() {
    let result = crate::configuration::load_configuration_from(map! {
        "PORT" => "65536",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::Port("65536".to_string()));
}

#[test]
fn port_0_should_not_be_accepted() {
    let result = crate::configuration::load_configuration_from(map! {
        "PORT" => "0",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::Port("0".to_string()));
}

#[test]
fn empty_service_port_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PORT" => "",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.port == Port(u16nz!(80)));
}

#[test]
fn blank_service_port_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PORT" => " ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.port == Port(u16nz!(80)));
}

#[test]
fn service_path_should_be_read_from_environment_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PATH" => "/this/is/the/path",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.path == Path::from("/this/is/the/path"));
}

#[test]
fn service_path_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {});

    let_assert!(Ok(configuration) = result);
    check!(configuration.path == Path::from("/"));
}

#[test]
fn empty_service_path_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PATH" => "",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.path == Path::from("/"));
}

#[test]
fn blank_service_path_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PATH" => " ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.path == Path::from("/"));
}

#[test]
fn service_path_should_be_trimmed() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_PATH" => " /this/is/the/path ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.path == Path::from("/this/is/the/path"));
}

#[test]
fn timeout_should_be_read_from_environment_variable() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => "100",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.timeout == Timeout(Duration::from_millis(100)));
}

#[test]
fn timeout_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {});

    let_assert!(Ok(configuration) = result);
    check!(configuration.timeout == Timeout(Duration::from_millis(500)));
}

#[test]
fn malformed_timeout_port_should_not_be_accepted() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => "MALFORMED",
    });

    let_assert!(Err(error) = result);
    check!(error == InvalidConfiguration::Timeout("MALFORMED".to_string()));
}

#[test]
fn empty_timeout_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => "",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.timeout == Timeout(Duration::from_millis(500)));
}

#[test]
fn blank_timeout_should_fallback_on_default() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => " ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.timeout == Timeout(Duration::from_millis(500)));
}

#[test]
fn timeout_should_be_trimmed() {
    let result = crate::configuration::load_configuration_from(map! {
        "DOCKTEUR_TIMEOUT_MILLIS" => " 100 ",
    });

    let_assert!(Ok(configuration) = result);
    check!(configuration.timeout == Timeout(Duration::from_millis(100)));
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Path(String::from(value))
    }
}
