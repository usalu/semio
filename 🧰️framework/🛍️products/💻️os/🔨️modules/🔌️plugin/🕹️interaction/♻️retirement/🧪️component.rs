//! 🧪️ Actual owned-state retirement laws at production and single-byte grants.
use super::*;

fn fixture() -> (serde_json::Value, serde_json::Value) {
    let cases = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.json")).unwrap();
    let retirement = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/♻️retirement.json")).unwrap();
    (cases, retirement)
}

fn close(cursor: &mut dyn ErasedSnapshotRetirement, bytes: usize) -> usize {
    let mut released = 0;
    assert!(matches!(cursor.close_step(0, bytes).unwrap(), SnapshotRetirementStep::Blocked));
    for _ in 0..200_000 {
        match cursor.close_step(1, bytes).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); assert!(released_bytes <= bytes); released += released_bytes; }
            SnapshotRetirementStep::Complete => { assert!(cursor.terminal_is_empty()); return released; }
            SnapshotRetirementStep::Blocked => panic!("positive interaction retirement grant blocked"),
        }
    }
    panic!("interaction retirement failed to terminate");
}

#[test]
fn local_interaction_retirement_matches_language_neutral_exact_bytes() {
    let (fixture, contract) = fixture();
    for grant in [1, 64, 4096] {
        for row in contract["cases"].as_array().unwrap() {
            let source = fixture["cases"].as_array().unwrap().iter().find(|case| case["id"] == row["sourceCase"]).unwrap();
            let mut value = source[row["sourceField"].as_str().unwrap()].clone();
            value["hover"] = serde_json::json!({});
            let state: InteractionState = serde_json::from_value(value).unwrap();
            let mut retirement = InteractionRetirement::owned(state);
            assert_eq!(close(&mut retirement, grant), row["expectedReleasedBytes"].as_u64().unwrap() as usize);
        }
    }
}

#[test]
fn local_interaction_retirement_shared_alias_and_final_owner_are_distinct() {
    let mut state = InteractionState::default();
    state.active_granularity.insert("domain".into(), "粒度🌊".repeat(1024));
    let expected = "domain".len() + "粒度🌊".len() * 1024;
    let root = Arc::new(state);
    let mut shared = InteractionRetirement::shared(root.clone());
    assert_eq!(close(&mut shared, 1), 0);
    assert_eq!(Arc::strong_count(&root), 1);
    let mut final_owner = InteractionRetirement::shared(root);
    assert_eq!(close(&mut final_owner, 1), expected);
}

#[test]
fn local_interaction_retirement_releases_empty_reserved_allocations() {
    let mut state = InteractionState::default();
    state.selection.insert("x".into(), DomainSelection { granularity: String::with_capacity(16384), ids: Vec::with_capacity(8192), anchor_id: Some(String::with_capacity(8192)) });
    state.hover.insert("y".into(), DomainHover { channel: String::with_capacity(16384), ids: Vec::with_capacity(8192) });
    let mut retirement = InteractionRetirement::owned(state);
    assert_eq!(close(&mut retirement, 1), 2);
    assert!(retirement.terminal_is_empty());
}

#[test]
fn local_interaction_retirement_live_drop_is_rejected() {
    assert!(std::panic::catch_unwind(|| { drop(InteractionRetirement::owned(InteractionState::default())); }).is_err());
}
