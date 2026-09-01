use super::*;
use crate::value::ordered::Grant;
use std::sync::Arc;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }
fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../../../🧪️fixtures/🏠️local-interaction/🔣️.json")).unwrap() }
fn grant(bytes: usize) -> Grant { Grant { maximum_items: 1, maximum_bytes: bytes } }
fn account(step: LocalInteractionUpdateStep, bytes: usize) -> usize {
    match step {
        LocalInteractionUpdateStep::Progress { completed_items, compared_bytes, released_items, released_bytes } => {
            assert!(completed_items + released_items <= 1);
            assert!(compared_bytes + released_bytes <= bytes);
            released_bytes
        }
        LocalInteractionUpdateStep::Complete => 0,
        LocalInteractionUpdateStep::Blocked => panic!("admitted update unexpectedly blocked"),
    }
}
fn close(mut cursor: LocalInteractionRootUpdate, bytes: usize) -> usize {
    cursor.begin_close();
    let mut released = 0;
    for _ in 0..500_000 {
        if cursor.terminal_is_empty() { return released; }
        released += account(cursor.close_step(grant(bytes)), bytes);
    }
    panic!("update owner failed to reach exact terminal");
}
fn retire(mut cursor: LocalInteractionRootRetirement, bytes: usize) -> usize {
    let mut released = 0;
    for _ in 0..500_000 {
        match cursor.advance(grant(bytes)) {
            LocalInteractionRootStep::Progress { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= bytes); released += released_bytes; }
            LocalInteractionRootStep::Complete => return released,
            LocalInteractionRootStep::Blocked => panic!("root close stalled"),
        }
    }
    panic!("root owner failed to close");
}

#[test]
fn local_interaction_retained_update_three_fields_are_atomic_and_exact() {
    let fixture = fixture(); let cases = cases();
    for name in fixture["cases"].as_array().unwrap() {
        let case = cases["cases"].as_array().unwrap().iter().find(|row| &row["id"] == name).unwrap();
        for bytes in [1, 64, 4096] {
            let root = LocalInteractionRoot::from_cold(serde_json::from_value(case["before"].clone()).unwrap());
            let patch = LocalInteractionRootPatch::from_cold(serde_json::from_value(case["restore"]["domains"]["graph"].clone()).unwrap());
            let pointer = patch.selection().map(Arc::as_ptr);
            let mut update = root.begin_domain_patch(Arc::new("graph".into()), patch);
            assert_eq!(update.advance(Grant { maximum_items: 0, maximum_bytes: bytes }), LocalInteractionUpdateStep::Blocked);
            assert_eq!(update.advance(grant(0)), LocalInteractionUpdateStep::Blocked);
            for _ in 0..10_000 {
                if update.is_complete() { break; }
                assert!(update.take().is_none());
                account(update.advance(grant(bytes)), bytes);
                assert_eq!(serde_json::to_value(&root).unwrap(), case["before"]);
            }
            assert!(update.is_complete());
            let result = update.take().unwrap();
            assert_eq!(serde_json::to_value(&result).unwrap(), case["expected"]);
            if let Some(pointer) = pointer { assert_eq!(result.selection().get("graph").unwrap() as *const _, pointer); }
            close(update, bytes); retire(root.retire(), bytes); retire(result.retire(), bytes);
        }
    }
}

#[test]
fn local_interaction_retained_update_every_cancel_frontier_retires_exact_owners() {
    let fixture = fixture(); let cases = cases();
    let case = cases["cases"].as_array().unwrap().iter().find(|row| row["id"] == fixture["cancelSourceCase"]).unwrap();
    for bytes in [1, 64, 4096] {
        let mut last_cut = false;
        for cut in 0..10_000 {
            let root = LocalInteractionRoot::from_cold(serde_json::from_value(case["before"].clone()).unwrap());
            let patch = LocalInteractionRootPatch::from_cold(serde_json::from_value(case["restore"]["domains"]["graph"].clone()).unwrap());
            let mut update = root.begin_domain_patch(Arc::new("graph".into()), patch);
            let mut released = retire(root.retire(), bytes);
            for _ in 0..cut { released += account(update.advance(grant(bytes)), bytes); }
            if update.is_complete() { last_cut = true; }
            update.begin_close(); assert!(update.take().is_none());
            released += close(update, bytes);
            assert_eq!(released as u64, fixture["cancelOwnedStringBytes"].as_u64().unwrap(), "cut {cut}, grant {bytes}");
            if last_cut { break; }
        }
        assert!(last_cut);
    }
}

#[test]
fn local_interaction_retained_update_long_keys_are_compared_under_each_byte_grant() {
    let fixture = fixture(); let cases = cases();
    let state: LocalInteractionState = serde_json::from_value(cases["cases"].as_array().unwrap().iter().find(|row| row["id"] == fixture["largeSourceCase"]).unwrap()["expected"].clone()).unwrap();
    for bytes in [1, 64, 4096] {
        let domain = Arc::new(state.selection.first_key_value().unwrap().0.clone());
        let key_bytes = domain.len(); assert!(key_bytes > 4096);
        let root = LocalInteractionRoot::from_cold(state.clone());
        let patch = LocalInteractionRootPatch::from_cold(LocalInteractionDomainPatch { selection: None, active_mode: None, active_granularity: None });
        let mut update = root.begin_domain_patch(domain, patch);
        let mut compared = 0;
        for _ in 0..500_000 {
            if update.is_complete() { break; }
            let step = update.advance(grant(bytes));
            if let LocalInteractionUpdateStep::Progress { compared_bytes, .. } = step { compared += compared_bytes; }
            account(step, bytes);
        }
        assert!(update.is_complete()); assert!(compared >= 2 * key_bytes);
        let result = update.take().unwrap(); assert!(result.selection().is_empty());
        close(update, bytes); retire(root.retire(), bytes); retire(result.retire(), bytes);
    }
}
