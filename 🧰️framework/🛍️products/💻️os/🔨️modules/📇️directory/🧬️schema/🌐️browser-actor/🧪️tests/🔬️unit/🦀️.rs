
use super::*;

#[test]
fn document_browser_actor_v1_matches_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️.json")).expect("neutral actor corpus");
    let source = DocumentBrowserActorSourceV1 { component_sha256: fixture["componentSha256"].as_str().unwrap(), descriptor_byte_sha256: fixture["descriptorByteSha256"].as_str().unwrap() };
    for law in fixture["cases"].as_array().unwrap() {
        let mut candidate = if law["kind"] == "none" { serde_json::json!({ "kind": "none" }) } else { fixture["closed"].clone() };
        if law["view"] == "lease" && law["kind"] == "closed" {
            candidate["byteLength"] = fixture["byteLength"].clone();
        }
        if let Some(fields) = law["set"].as_object() {
            candidate.as_object_mut().unwrap().extend(fields.clone());
        }
        if let Some(field) = law["remove"].as_str() {
            candidate.as_object_mut().unwrap().remove(field);
        }
        let json = serde_json::to_string(&candidate).unwrap();
        let renderer = law["renderer"].as_str().unwrap();
        let accepted = if law["view"] == "plan" {
            match crate::os_pack::json::from_json_str::<DocumentOpenBrowserActorV1>(&json) {
                Ok(value) if value.validate(source, renderer).is_ok() => {
                    assert_eq!(serde_json::from_str::<serde_json::Value>(&crate::os_pack::json::to_json_string(&value)).unwrap(), candidate);
                    true
                }
                _ => false,
            }
        } else {
            match crate::os_pack::json::from_json_str::<DocumentExecutionTargetBrowserActorV1>(&json) {
                Ok(value) if value.validate(source, renderer).is_ok() => {
                    assert_eq!(serde_json::from_str::<serde_json::Value>(&crate::os_pack::json::to_json_string(&value)).unwrap(), candidate);
                    true
                }
                _ => false,
            }
        };
        assert_eq!(accepted, law["accepted"].as_bool().unwrap(), "{}", law["id"]);
    }
    let plan: DocumentOpenBrowserActorV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&fixture["closed"]).unwrap()).unwrap();
    let length = fixture["byteLength"].as_u64().unwrap();
    let lease = plan.to_lease(source, "wasm", Some(length)).unwrap();
    let encoded: serde_json::Value = serde_json::from_str(&crate::os_pack::json::to_json_string(&lease)).unwrap();
    let mut expected = fixture["closed"].clone();
    expected["byteLength"] = fixture["byteLength"].clone();
    assert_eq!(encoded, expected);
    for field in expected.as_object().unwrap().keys() {
        let mut changed = expected.clone();
        if field == "kind" {
            changed = serde_json::json!({ "kind": "none" });
        } else if field == "importInterfaces" {
            changed[field] = serde_json::json!([]);
        } else if field == "byteLength" {
            changed[field] = serde_json::json!(length + 1);
        } else {
            changed[field] = serde_json::json!("different");
        }
        let other: DocumentExecutionTargetBrowserActorV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&changed).unwrap()).unwrap();
        assert_ne!(lease, other, "equality omitted {field}");
    }
    assert_eq!(DocumentOpenBrowserActorV1::None.to_lease(source, "react", None), Ok(DocumentExecutionTargetBrowserActorV1::None));
    assert!(DocumentOpenBrowserActorV1::None.to_lease(source, "react", Some(1)).is_err());
    assert!(DocumentOpenBrowserActorV1::None.to_lease(source, "wasm", None).is_err());
    for length in [None, Some(0), Some(DOCUMENT_BROWSER_ACTOR_MAX_BYTES + 1)] {
        assert!(plan.to_lease(source, "wasm", length).is_err());
    }
    assert!(plan.to_lease(source, "react", Some(length)).is_err());
    assert!(plan.to_lease(DocumentBrowserActorSourceV1 { component_sha256: source.descriptor_byte_sha256, ..source }, "wasm", Some(length)).is_err());
    assert!(plan.to_lease(DocumentBrowserActorSourceV1 { descriptor_byte_sha256: source.component_sha256, ..source }, "wasm", Some(length)).is_err());
}
