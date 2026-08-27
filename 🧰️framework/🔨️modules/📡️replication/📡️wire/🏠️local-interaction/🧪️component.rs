//! 🧪️ Shared cold wire/authority laws; live retained query and publication tests are separate.
use super::*;

#[test]
fn local_interaction_language_neutral_restore_parity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️local-interaction.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let before: LocalInteractionState = serde_json::from_value(row["before"].clone()).unwrap();
        let current: LocalInteractionIdentity = serde_json::from_value(row["current"].clone()).unwrap();
        let restore: LocalInteractionRestore = serde_json::from_value(row["restore"].clone()).unwrap();
        let actual = restore.apply_cold(&before, &current);
        if row.get("error").is_some() { assert_eq!(actual, Err("stale-authority")); }
        else { assert_eq!(serde_json::to_value(actual.unwrap()).unwrap(), row["expected"]); }
        assert_eq!(serde_json::to_value(before).unwrap(), row["before"]);
        assert_eq!(serde_json::to_value(restore).unwrap(), row["restore"]);
    }
}

#[test]
fn local_interaction_wire_preserves_full_identity_and_u64() {
    let identity = LocalInteractionIdentity { app_instance_id: u32::MAX, generation: u64::MAX, revision: [1; 32], document_revision: [2; 32], topology_revision: [3; 32] };
    let encoded = serde_json::to_value(&identity).unwrap();
    assert_eq!(encoded["generation"], "18446744073709551615");
    assert_eq!(encoded["revision"].as_str().unwrap().len(), 64);
    assert_eq!(serde_json::from_value::<LocalInteractionIdentity>(encoded.clone()).unwrap(), identity);
    for invalid in [serde_json::json!(9007199254740993u64), serde_json::json!("18446744073709551616"), serde_json::json!("01"), serde_json::json!("+1")] {
        let mut value = encoded.clone(); value["generation"] = invalid;
        assert!(serde_json::from_value::<LocalInteractionIdentity>(value).is_err());
    }
    let mut truncated = encoded; truncated["documentRevision"] = serde_json::json!("0000000000000000");
    assert!(serde_json::from_value::<LocalInteractionIdentity>(truncated).is_err());
}

#[test]
fn local_interaction_nullable_fields_are_explicit_and_unknown_fields_fail() {
    let complete = serde_json::json!({"selection": null, "activeMode": null, "activeGranularity": null});
    assert!(serde_json::from_value::<LocalInteractionDomainPatch>(complete.clone()).is_ok());
    for field in ["selection", "activeMode", "activeGranularity"] {
        let mut value = complete.clone(); value.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<LocalInteractionDomainPatch>(value).is_err());
    }
    let mut extra = complete; extra["selectionJson"] = serde_json::json!("{}");
    assert!(serde_json::from_value::<LocalInteractionDomainPatch>(extra).is_err());
    assert!(serde_json::from_value::<LocalInteractionState>(serde_json::json!({"selection": {}, "activeMode": {}, "activeGranularity": {}, "hover": {}})).is_err());
}
