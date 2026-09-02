use super::*;
use crate::value::ordered::Grant;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }
fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../../🧪️fixtures/🏠️local-interaction/🔣️.json")).unwrap() }

fn drain(mut owner: LocalInteractionRootRetirement, bytes: usize) -> usize {
    assert_eq!(owner.advance(Grant { maximum_items: 0, maximum_bytes: bytes }), LocalInteractionRootStep::Blocked);
    assert_eq!(owner.advance(Grant { maximum_items: 1, maximum_bytes: 0 }), LocalInteractionRootStep::Blocked);
    let mut retired = 0;
    for _ in 0..200_000 {
        match owner.advance(Grant { maximum_items: 1, maximum_bytes: bytes }) {
            LocalInteractionRootStep::Blocked => panic!("admitted root retirement stalled"),
            LocalInteractionRootStep::Progress { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= bytes); retired += released_bytes; }
            LocalInteractionRootStep::Complete => { assert!(owner.terminal_is_empty()); return retired; }
        }
    }
    panic!("retained root failed to close");
}

#[test]
fn local_interaction_retained_root_preserves_exact_wire_and_shared_payloads() {
    let fixture = fixture();
    let cases = cases();
    let source = &cases["cases"].as_array().unwrap().iter().find(|case| case["id"] == fixture["sourceCase"]).unwrap()["before"];
    for bytes in [1, 64, 4096] {
        let root = LocalInteractionRoot::from_cold(super::from_json(source.clone()));
        let shared = root.clone();
        assert!(std::ptr::eq(root.selection().first_key_value().unwrap().1, shared.selection().first_key_value().unwrap().1));
        assert_eq!(super::dsl_to_json(&crate::value::ToValue::to_value(&root)), *source);
        assert_eq!(drain(root.retire(), bytes), fixture["sharedOwnerRetiredBytes"].as_u64().unwrap() as usize);
        assert_eq!(super::dsl_to_json(&crate::value::ToValue::to_value(&shared)), *source);
        assert_eq!(drain(shared.retire(), bytes), fixture["finalOwnerRetiredBytes"].as_u64().unwrap() as usize);
    }
}

#[test]
fn local_interaction_retained_root_large_semantic_content_closes_under_actual_grants() {
    let fixture = fixture();
    let cases = cases();
    let source = &cases["cases"].as_array().unwrap().iter().find(|case| case["id"] == fixture["largeSourceCase"]).unwrap()["expected"];
    for bytes in [1, 64, 4096] {
        let root = LocalInteractionRoot::from_cold(super::from_json(source.clone()));
        assert_eq!(super::dsl_to_json(&crate::value::ToValue::to_value(&root)), *source);
        assert_eq!(drain(root.retire(), bytes), fixture["largeFinalOwnerRetiredBytes"].as_u64().unwrap() as usize);
    }
}
