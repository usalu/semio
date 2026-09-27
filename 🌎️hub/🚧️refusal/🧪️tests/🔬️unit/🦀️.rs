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

/// 🪪️ The Rust credential refusal is exactly `HubCredentialRefusalV1` with `HubCredentialRefusalMessageV1`: the served
/// body serializes to a value whose schema, status, code and both notices are the declared constants.
#[test]
fn the_credential_refusal_is_the_declared_schema_body() {
    let module: serde_json::Value = serde_json::from_str(HUB_REFUSAL_SCHEMA_JSON).unwrap();
    let declared = &module["$defs"]["HubCredentialRefusalV1"]["properties"];
    let body = serde_json::to_value(HUB_CREDENTIAL_REFUSAL).unwrap();
    assert_eq!(body["schema"], declared["schema"]["const"]);
    assert_eq!(body["status"], declared["status"]["const"]);
    assert_eq!(body["code"], declared["code"]["const"]);
    assert_eq!(body["message"], module["$defs"]["HubCredentialRefusalMessageV1"]["const"]);
    assert_eq!(body.as_object().unwrap().len(), 4);
    assert_eq!(refusal_code(HUB_CREDENTIAL_REFUSAL.status), Some(HUB_CREDENTIAL_REFUSAL.code));
}

/// ⏳️ The Rust transient-apply refusal is `HubTransientApplyRefusalCodeV1` / `HubTransientApplyRefusalMessageV1`: the code
/// is the declared const and every reason becomes a non-empty message within the declared bound.
#[test]
fn the_transient_apply_refusal_is_the_declared_schema_message() {
    let module: serde_json::Value = serde_json::from_str(HUB_REFUSAL_SCHEMA_JSON).unwrap();
    assert_eq!(module["$defs"]["HubTransientApplyRefusalCodeV1"]["const"], HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    let message = &module["$defs"]["HubTransientApplyRefusalMessageV1"]["properties"]["message"];
    assert_eq!(message["maxLength"].as_u64(), Some(HUB_TRANSIENT_APPLY_REFUSAL_MESSAGE_MAX_CHARS as u64));
    assert_eq!(hub_transient_apply_refusal_message("DB I/O aggregate admission exhausted"), "DB I/O aggregate admission exhausted");
    assert_eq!(hub_transient_apply_refusal_message(""), HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    assert_eq!(hub_transient_apply_refusal_message(&"é".repeat(5000)).chars().count(), HUB_TRANSIENT_APPLY_REFUSAL_MESSAGE_MAX_CHARS);
}
