use std::time::Duration;
use std::collections::HashMap;
use std::num::NonZeroU16;
use std::str::FromStr;

#[cfg(test)]
#[path = "./configuration_test.rs"]
mod test;

#[cfg(test)]
#[path = "./configuration_fixtures.rs"]
pub(crate) mod fixtures;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct Method(http::Method);

impl From<Method> for reqwest::Method {

    fn from(value: Method) -> Self {
        value.0
    }
}

impl Default for Method {

    fn default() -> Self {
        Method(http::Method::GET)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct Port(NonZeroU16);

impl From<Port> for u16 {

    fn from(value: Port) -> Self {
        value.0.into()
    }
}

impl Default for Port {

    fn default() -> Self {
        Port(NonZeroU16::new(80).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct Path(String);

impl From<Path> for String {

    fn from(value: Path) -> Self {
        value.0
    }
}

impl Default for Path {

    fn default() -> Self {
        Path(String::from("/"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct Timeout(Duration);

impl From<Timeout> for Duration {

    fn from(value: Timeout) -> Self {
        value.0
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout(Duration::from_millis(500))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct StatusCode(NonZeroU16);

impl PartialEq<StatusCode> for http::status::StatusCode {

    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.0.get()
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode(NonZeroU16::new(200).unwrap())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Configuration {
    pub(crate) method: Method,
    pub(crate) port: Port,
    pub(crate) path: Path,
    pub(crate) timeout: Timeout,
    pub(crate) status_code: StatusCode,
}

#[derive(Debug, PartialEq)]
pub(crate) enum InvalidConfiguration {
    Port(String),
    Timeout(String),
    StatusCode(String),
    Method(String),
}

#[macro_export]
macro_rules! env {
    ( $x:expr ) => {
        format!("DOCKTEUR_{}", $x).as_str()
    };
}

pub fn sanitize(value: &str) -> Option<String> {
    Some(value.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn load_method_from(vars: &HashMap<String, String>) -> Result<Method, InvalidConfiguration> {
    match vars.get(env!("METHOD")) {
        None => Ok(Method::default()),
        Some(value) => match sanitize(value) {
            None => Ok(Method::default()),
            Some(value) => http::Method::from_str(value.as_str())
                .map(Method)
                .map_err(|_| InvalidConfiguration::Method(value)),
        },
    }
}

fn load_port_from(vars: &HashMap<String, String>) -> Result<Port, InvalidConfiguration> {
    let env_var = vars.get(env!("PORT"))
        .or(vars.get("PORT"));

    match env_var {
        None => Ok(Port::default()),
        Some(value) => match sanitize(value) {
            None => Ok(Port::default()),
            Some(value) => match value.parse::<u16>() {
                Ok(number) => match NonZeroU16::new(number) {
                    None => Err(InvalidConfiguration::Port(value.clone())),
                    Some(_) => Ok(Port(NonZeroU16::new(number).unwrap())),
                },
                Err(_) => Err(InvalidConfiguration::Port(value.clone())),
            }
        },
    }
}

fn load_path_from(vars: &HashMap<String, String>) -> Result<Path, InvalidConfiguration> {
    match vars.get(env!("PATH")) {
        None => Ok(Path::default()),
        Some(value) => match sanitize(value) {
            None => Ok(Path::default()),
            Some(value) => Ok(Path(value)),
        },
    }
}

fn load_timeout_from(vars: &HashMap<String, String>) -> Result<Timeout, InvalidConfiguration> {
    match vars.get(env!("TIMEOUT_MILLIS")) {
        None => Ok(Timeout::default()),
        Some(value) => match sanitize(value) {
            None => Ok(Timeout::default()),
            Some(value) => match value.parse::<u64>() {
                Ok(value) => Ok(Timeout(Duration::from_millis(value))),
                Err(_) => Err(InvalidConfiguration::Timeout(value)),
            }
        },
    }
}

fn load_status_code_from(vars: &HashMap<String, String>) -> Result<StatusCode, InvalidConfiguration> {
    match vars.get(env!("STATUS_CODE")) {
        None => Ok(StatusCode::default()),
        Some(value) => match sanitize(value) {
            None => Ok(StatusCode::default()),
            Some(string_value) => match string_value.parse::<u16>() {
                Ok(value) => match NonZeroU16::new(value) {
                    Some(value) => Ok(StatusCode(value)),
                    None => Err(InvalidConfiguration::StatusCode(string_value)),
                },
                Err(_) => Err(InvalidConfiguration::StatusCode(string_value)),
            }
        },
    }
}

pub(crate) fn load_configuration_from(vars: HashMap<String, String>) -> Result<Configuration, InvalidConfiguration> {
    let method = load_method_from(&vars)?;
    let port = load_port_from(&vars)?;
    let path = load_path_from(&vars)?;
    let timeout = load_timeout_from(&vars)?;
    let status_code = load_status_code_from(&vars)?;
    Ok(Configuration { method, port, path, timeout, status_code })
}
