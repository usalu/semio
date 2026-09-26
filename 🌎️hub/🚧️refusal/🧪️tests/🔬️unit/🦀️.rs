use super::*;

/// 🗺️ The Rust status table is exactly `HubRefusalStatusCodesV1`, and every code it or the fallback can
/// produce is a member of `HubRefusalCodeV1`.
#[test]
fn refusal_codes_are_the_declared_schema_table() {
    let module: serde_json::Value = serde_json::from_str(HUB_REFUSAL_SCHEMA_JSON).unwrap();
    let declared = module["$defs"]["HubRefusalStatusCodesV1"]["const"].as_object().unwrap();
    let codes: Vec<&str> = module["$defs"]["HubRefusalCodeV1"]["enum"].as_array().unwrap().iter().map(|code| code.as_str().unwrap()).collect();
    assert_eq!(declared.len(), HUB_REFUSAL_STATUS_CODES.len());
    for (status, code) in HUB_REFUSAL_STATUS_CODES {
        assert_eq!(declared[&status.to_string()].as_str(), Some(code), "{status}");
        assert!(codes.contains(&code), "{code}");
    }
    assert!(codes.contains(&"refused") && codes.contains(&"insecure-transport"));
    assert_eq!(refusal_code(200), None);
    assert_eq!(refusal_code(399), None);
    assert_eq!(refusal_code(418), Some("refused"));
    assert_eq!(refusal_code(599), Some("refused"));
    assert_eq!(refusal_code(413), Some("payload-too-large"));
    assert_eq!(module["$defs"]["HubRefusalV1"]["properties"]["schema"]["const"], HUB_REFUSAL_SCHEMA);
}
