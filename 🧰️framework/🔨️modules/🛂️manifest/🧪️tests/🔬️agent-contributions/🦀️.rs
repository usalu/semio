
use super::*;

#[semio_framework_async_macros::async_test]
async fn default_is_empty_and_promoted_subset_holds_trivially() {
    let contributions = AgentContributions::default();
    assert!(contributions.capabilities.is_empty());
    assert!(contributions.promoted.is_empty());
    assert!(contributions.promoted_is_subset_of_capabilities().await);
}

#[semio_framework_async_macros::async_test]
async fn promoted_subset_of_capabilities_holds_and_is_violated_correctly() {
    let ok = AgentContributions { capabilities: vec!["note.editor.deleteSelection".into()], promoted: vec!["note.editor.deleteSelection".into()] };
    assert!(ok.promoted_is_subset_of_capabilities().await);
    let bad = AgentContributions { capabilities: vec!["note.editor.deleteSelection".into()], promoted: vec!["note.editor.addBlock".into()] };
    assert!(!bad.promoted_is_subset_of_capabilities().await);
}

#[semio_framework_async_macros::async_test]
async fn canonical_json_round_trip_uses_camel_case_and_skips_empty_promoted() {
    let contributions = AgentContributions { capabilities: vec!["note.editor.deleteSelection".into()], promoted: vec![] };
    let text = dsl::os_pack::json::to_json_string(&contributions);
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json, serde_json::json!({ "capabilities": ["note.editor.deleteSelection"] }));
    let round_tripped: AgentContributions = dsl::os_pack::json::from_json_str(&text).unwrap();
    assert_eq!(round_tripped, contributions);
}

#[semio_framework_async_macros::async_test]
async fn never_conflated_with_capability_requests() {
    // 🚨️ `AgentContributions.capabilities` (what this package OFFERS) and
    // `PackageDescriptor.capability_requests: Vec<kernel::CapabilityRequest>` (what this
    // package NEEDS) are different types with different shapes — this test exists only to
    // pin the distinction in code, not just in the doc comment above, so a future edit that
    // tries to merge them fails to compile rather than silently drifting.
    let offers = AgentContributions { capabilities: vec!["note.editor.deleteSelection".into()], promoted: vec![] };
    let needs = kernel::CapabilityRequest { id: kernel::CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist edits".into(), optional: false };
    assert_ne!(offers.capabilities.first().map(String::as_str), Some(needs.id.0.as_str()));
}
