//! 🧪️ Transparent wire, O(1) root sharing, and byte-bounded final-owner retirement.

use super::*;
use crate::os_store as store;
use store::ErasedSnapshotRetirement;

//#region 🧪️AllocationAndRetirement
#[test]
fn generation_root_large_json_wire_and_shared_allocation_survive_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    for maximum in [1, 4096] {
        let root: GenerationPlayRoot = serde_json::from_value(fixture["generation"].clone()).unwrap();
        assert_eq!(serde_json::to_value(&root).unwrap(), fixture["generation"]);
        let copy = root.clone();
        assert!(root.same_allocation(&copy));
        let weak = Arc::downgrade(root.0.as_ref().unwrap());
        let mut first = root.into_retirement();
        while !matches!(first.close_step(1, maximum).unwrap(), store::SnapshotRetirementStep::Complete) {}
        assert!(first.terminal_is_empty());
        assert!(weak.upgrade().is_some());
        assert_eq!(serde_json::to_value(&copy).unwrap(), fixture["generation"]);
        let mut last = copy.into_retirement();
        assert!(matches!(last.close_step(1, 0).unwrap(), store::SnapshotRetirementStep::Blocked));
        for _ in 0..100_000 {
            match last.close_step(1, maximum).unwrap() {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= maximum),
                store::SnapshotRetirementStep::Complete => break,
                store::SnapshotRetirementStep::Blocked => panic!("positive generation retirement grant blocked"),
            }
        }
        assert!(last.terminal_is_empty());
        assert!(weak.upgrade().is_none());
    }
}

#[test]
fn generation_root_cold_builder_refuses_shared_mutation_without_cloning() {
    let mut root = GenerationPlayRoot::default();
    root.cold_builder_mut().unwrap().preview_text = Some("mutable cold builder".into());
    let shared = root.clone();
    assert_eq!(root.cold_builder_mut().unwrap_err(), "playbook.generation-root-shared");
    assert!(root.same_allocation(&shared));
    let mut first = root.into_retirement();
    let mut second = shared.into_retirement();
    while !matches!(first.close_step(1, 1).unwrap(), store::SnapshotRetirementStep::Complete) {}
    while !matches!(second.close_step(1, 1).unwrap(), store::SnapshotRetirementStep::Complete) {}
}
//#endregion 🧪️AllocationAndRetirement

//#region 🧪️LifecycleGuards
#[test]
fn generation_root_live_final_owner_and_unclosed_cursor_reject_drop_without_double_panic() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    let root: GenerationPlayRoot = serde_json::from_value(fixture["generation"].clone()).unwrap();
    assert!(std::panic::catch_unwind(|| drop(root)).is_err());
    let root: GenerationPlayRoot = serde_json::from_value(fixture["generation"].clone()).unwrap();
    assert!(std::panic::catch_unwind(|| drop(root.into_retirement())).is_err());
    let root: GenerationPlayRoot = serde_json::from_value(fixture["generation"].clone()).unwrap();
    assert!(std::thread::spawn(move || {
        let _retirement = root.into_retirement();
        panic!("primary generation lifecycle fault");
    }).join().is_err());
}

#[test]
fn generation_root_close_resumes_every_phase_and_zero_grants_preserve_exact_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    for pause in 0..12 {
        let root: GenerationPlayRoot = serde_json::from_value(fixture["generation"].clone()).unwrap();
        let mut retirement = root.into_retirement();
        for _ in 0..pause { retirement.close_step(1, 1).unwrap(); }
        assert!(matches!(retirement.close_step(0, 4096).unwrap(), store::SnapshotRetirementStep::Blocked));
        assert!(matches!(retirement.close_step(1, 0).unwrap(), store::SnapshotRetirementStep::Blocked));
        let retirement = std::thread::spawn(move || {
            for _ in 0..100_000 {
                if matches!(retirement.close_step(1, 4096).unwrap(), store::SnapshotRetirementStep::Complete) { break; }
            }
            retirement
        }).join().unwrap();
        assert!(retirement.terminal_is_empty());
    }
}
//#endregion 🧪️LifecycleGuards
