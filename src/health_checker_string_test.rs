use crate::health_checker::sanitize;

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
