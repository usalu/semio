//! 🧪️ Shared cold wire/authority laws; live retained query and publication tests are separate.
use super::*;

#[test]
fn local_interaction_language_neutral_restore_parity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let before: LocalInteractionState = super::from_json(row["before"].clone());
        let current: LocalInteractionIdentity = super::from_json(row["current"].clone());
        let restore: LocalInteractionRestore = super::from_json(row["restore"].clone());
        let actual = restore.apply_cold(&before, &current);
        if row.get("error").is_some() { assert_eq!(actual, Err("stale-authority")); }
        else { assert_eq!(super::dsl_to_json(&crate::value::ToValue::to_value(&actual.unwrap())), row["expected"]); }
        assert_eq!(super::dsl_to_json(&crate::value::ToValue::to_value(&before)), row["before"]);
        assert_eq!(super::dsl_to_json(&crate::value::ToValue::to_value(&restore)), row["restore"]);
    }
}

#[test]
fn local_interaction_wire_preserves_full_identity_and_u64() {
    let identity = LocalInteractionIdentity { app_instance_id: u32::MAX, generation: u64::MAX, revision: [1; 32], document_revision: [2; 32], topology_revision: [3; 32] };
    let encoded = super::dsl_to_json(&crate::value::ToValue::to_value(&identity));
    assert_eq!(encoded["generation"], "18446744073709551615");
    assert_eq!(encoded["revision"].as_str().unwrap().len(), 64);
    assert_eq!(super::from_json::<LocalInteractionIdentity>(encoded.clone()), identity);
    for invalid in [serde_json::json!(9007199254740993u64), serde_json::json!("18446744073709551616"), serde_json::json!("01"), serde_json::json!("+1")] {
        let mut value = encoded.clone(); value["generation"] = invalid;
        assert!(<LocalInteractionIdentity as crate::value::FromValue>::from_value(super::json_to_dsl(value)).is_err());
    }
    let mut truncated = encoded; truncated["documentRevision"] = serde_json::json!("0000000000000000");
    assert!(<LocalInteractionIdentity as crate::value::FromValue>::from_value(super::json_to_dsl(truncated)).is_err());
}

#[test]
fn local_interaction_nullable_fields_are_explicit_and_unknown_fields_fail() {
    let complete = serde_json::json!({"selection": null, "activeMode": null, "activeGranularity": null});
    assert!(<LocalInteractionDomainPatch as crate::value::FromValue>::from_value(super::json_to_dsl(complete.clone())).is_ok());
    for field in ["selection", "activeMode", "activeGranularity"] {
        let mut value = complete.clone(); value.as_object_mut().unwrap().remove(field);
        assert!(<LocalInteractionDomainPatch as crate::value::FromValue>::from_value(super::json_to_dsl(value)).is_err());
    }
    let mut extra = complete; extra["selectionJson"] = serde_json::json!("{}");
    assert!(<LocalInteractionDomainPatch as crate::value::FromValue>::from_value(super::json_to_dsl(extra)).is_err());
    assert!(<LocalInteractionState as crate::value::FromValue>::from_value(super::json_to_dsl(serde_json::json!({"selection": {}, "activeMode": {}, "activeGranularity": {}, "hover": {}}))).is_err());
}
