use hyperlane_utils::*;

#[test]
fn test_json_parse_from_str() {
    let json_str: &'static str = r#"{"name":"test","value":123}"#;
    let result: serde_json::Value = json_parse_from_str(json_str).unwrap();
    assert_eq!(result["name"], "test");
    assert_eq!(result["value"], 123);
}

#[test]
fn test_json_stringify_string() {
    let data: serde_json::Value = serde_json::json!({"name": "test", "value": 123});
    let json_str: String = json_stringify_string(&data).unwrap();
    assert!(json_str.contains("\"name\":\"test\""));
    assert!(json_str.contains("\"value\":123"));
}
