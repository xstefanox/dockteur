use std::time::Duration;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use crate::health_checker::{Configuration, ConfigurationError, get_health, load_configuration, State::Healthy};
use crate::health_checker::State::Unhealthy;

#[test]
fn service_port_should_be_read_from_environment_variable() {
    temp_env::with_var("HEALTHCHECK_PORT", Some("8080"), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.port, 8080);
    });
}

#[test]
fn service_port_should_fallback_on_default() {
    let configuration = load_configuration().unwrap();

    assert_eq!(configuration.port, 80);
}

#[test]
fn malformed_service_port_should_not_be_accepted() {
    temp_env::with_var("HEALTHCHECK_PORT", Some("MALFORMED"), || {
        let result = load_configuration().unwrap_err();

        assert_eq!(result, ConfigurationError::InvalidPort("MALFORMED".to_string()));
    });
}

#[test]
fn service_path_should_be_read_from_environment_variable() {
    temp_env::with_var("HEALTHCHECK_PATH", Some("/this/is/the/path"), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.path, "/this/is/the/path");
    });
}

#[test]
fn service_path_should_fallback_on_default() {
    let configuration = load_configuration().unwrap();

    assert_eq!(configuration.path, "/");
}

#[test]
fn empty_service_path_should_fallback_on_default() {
    temp_env::with_var("HEALTHCHECK_PATH", Some(""), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.path, "/");
    });
}

#[test]
fn blank_service_path_should_fallback_on_default() {
    temp_env::with_var("HEALTHCHECK_PATH", Some(" "), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.path, "/");
    });
}

#[test]
fn service_path_should_be_trimmed() {
    temp_env::with_var("HEALTHCHECK_PATH", Some(" /this/is/the/path "), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.path, "/this/is/the/path");
    });
}

#[test]
fn timeout_should_be_read_from_environment_variable() {
    temp_env::with_var("HEALTHCHECK_TIMEOUT_MILLIS", Some("100"), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.timeout, Duration::from_millis(100));
    });
}

#[test]
fn timeout_path_should_fallback_on_default() {
    let configuration = load_configuration().unwrap();

    assert_eq!(configuration.timeout, Duration::from_millis(500));
}

#[test]
fn malformed_timeout_port_should_not_be_accepted() {
    temp_env::with_var("HEALTHCHECK_TIMEOUT_MILLIS", Some("MALFORMED"), || {
        let result = load_configuration().unwrap_err();

        assert_eq!(result, ConfigurationError::InvalidTimeout("MALFORMED".to_string()));
    });
}

#[test]
fn empty_timeout_should_fallback_on_default() {
    temp_env::with_var("HEALTHCHECK_TIMEOUT_MILLIS", Some(""), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.timeout, Duration::from_millis(500));
    });
}

#[test]
fn blank_timeout_should_fallback_on_default() {
    temp_env::with_var("HEALTHCHECK_TIMEOUT_MILLIS", Some(" "), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.timeout, Duration::from_millis(500));
    });
}

#[test]
fn timeout_should_be_trimmed() {
    temp_env::with_var("HEALTHCHECK_TIMEOUT_MILLIS", Some(" 100 "), || {
        let configuration = load_configuration().unwrap();

        assert_eq!(configuration.timeout, Duration::from_millis(100));
    });
}

#[tokio::test]
async fn a_healthy_service_should_be_reported() {
    let service_path = "/health";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(service_path))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let configuration = Configuration {
        port: mock_server.address().port(),
        path: service_path.to_string(),
        timeout: Duration::from_millis(100),
    };

    let state = get_health(&configuration);

    assert_eq!(Healthy, state);
}

#[tokio::test]
async fn an_unhealthy_service_should_be_reported() {
    let service_path = "/health";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(service_path))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server)
        .await;

    let configuration = Configuration {
        port: mock_server.address().port(),
        path: service_path.to_string(),
        timeout: Duration::from_millis(100),
    };

    let state = get_health(&configuration);

    assert_eq!(Unhealthy, state);
}

#[tokio::test]
async fn service_responding_slowly_should_be_reported_as_unhealthy() {
    let service_path = "/health";
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(service_path))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(10)))
        .expect(1)
        .mount(&mock_server)
        .await;

    let configuration = Configuration {
        port: mock_server.address().port(),
        path: service_path.to_string(),
        timeout: Duration::from_millis(1),
    };

    let state = get_health(&configuration);

    assert_eq!(Unhealthy, state);
}

#[test]
fn on_network_error_the_service_should_be_reported_as_unhealthy() {
    let configuration = Configuration {
        port: 8080,
        path: "/health".to_string(),
        timeout: Duration::from_millis(1),
    };

    let state = get_health(&configuration);

    assert_eq!(Unhealthy, state);
}
