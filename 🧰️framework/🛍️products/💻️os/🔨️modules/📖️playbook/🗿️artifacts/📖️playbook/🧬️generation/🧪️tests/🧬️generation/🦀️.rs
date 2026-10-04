//! 🧪️ Transparent wire, O(1) root sharing, and byte-bounded final-owner retirement.

use super::*;
use crate::os_store as store;
use store::ErasedSnapshotRetirement;

#[test]
fn generation_values_ranked_source_shares_payload_and_retires_every_alias_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["rankedValues"];
    let expected: Vec<String> = serde_json::from_value(policy["expectedKeys"].clone()).unwrap();
    for order in policy["aliasReleaseOrders"].as_array().unwrap() {
        let values: crate::PlaybookValues = policy["entries"].as_array().unwrap().iter().map(|entry| (entry[0].as_str().unwrap().to_owned(), entry[1].clone().into())).collect();
        let shared = values.clone();
        for (index, key) in expected.iter().enumerate() {
            let original = values.entry_at_rank(index).unwrap();
            let alias = shared.entry_at_rank(index).unwrap();
            assert_eq!(original.0, key);
            assert!(std::ptr::eq(original.0, alias.0));
            assert!(std::ptr::eq(original.1, alias.1));
        }
        assert!(values.entry_at_rank(expected.len()).is_none());
        let reference: serde_json::Map<String, serde_json::Value> = policy["entries"].as_array().unwrap().iter().map(|entry| (entry[0].as_str().unwrap().to_owned(), entry[1].clone())).collect();
        let generation = crate::FormGeneration { id: "ranked".into(), name: "Ranked".into(), values };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&generation)).unwrap()["values"], serde_json::Value::Object(reference));
        let root = GenerationPlayRoot::from(GenerationPlayState { generations: vec![generation], selected_generation_id: None, preview_text: None });
        let second = GenerationPlayRoot::from(GenerationPlayState { generations: vec![crate::FormGeneration { id: "ranked".into(), name: "Ranked".into(), values: shared }], selected_generation_id: None, preview_text: None });
        let third = root.clone();
        let mut aliases = [Some(root), Some(second), Some(third)];
        let mut turns = 0;
        for index in order.as_array().unwrap() {
            let mut close = aliases[index.as_u64().unwrap() as usize].take().unwrap().into_retirement();
            assert!(matches!(close.close_step(0, 3).unwrap(), store::SnapshotRetirementStep::Blocked));
            assert!(matches!(close.close_step(1, 0).unwrap(), store::SnapshotRetirementStep::Blocked));
            for _ in 0..100_000 {
                turns += 1;
                match close.close_step(1, 3).unwrap() {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 3),
                    store::SnapshotRetirementStep::Complete => break,
                    store::SnapshotRetirementStep::Blocked => panic!("positive ranked values release blocked"),
                }
            }
            assert!(close.terminal_is_empty());
        }
        eprintln!("[DEBUG] ranked generation alias order={order} closeTurns={turns}");
    }
}

//#region 🧪️AllocationAndRetirement
#[test]
fn generation_root_large_json_wire_and_shared_allocation_survive_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for maximum in [1, 4096] {
        let root: GenerationPlayRoot = semio_framework_pack_json::from_json_str(&fixture["generation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&root)).unwrap(), fixture["generation"]);
        let copy = root.clone();
        assert!(root.same_allocation(&copy));
        let weak = Arc::downgrade(root.0.as_ref().unwrap());
        let mut first = root.into_retirement();
        while !matches!(first.close_step(1, maximum).unwrap(), store::SnapshotRetirementStep::Complete) {}
        assert!(first.terminal_is_empty());
        assert!(weak.upgrade().is_some());
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&copy)).unwrap(), fixture["generation"]);
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let root: GenerationPlayRoot = semio_framework_pack_json::from_json_str(&fixture["generation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(std::panic::catch_unwind(|| drop(root)).is_err());
    let root: GenerationPlayRoot = semio_framework_pack_json::from_json_str(&fixture["generation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(std::panic::catch_unwind(|| drop(root.into_retirement())).is_err());
    let root: GenerationPlayRoot = semio_framework_pack_json::from_json_str(&fixture["generation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(std::thread::spawn(move || {
        let _retirement = root.into_retirement();
        panic!("primary generation lifecycle fault");
    })
    .join()
    .is_err());
}

#[test]
fn generation_root_close_resumes_every_phase_and_zero_grants_preserve_exact_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for pause in 0..12 {
        let root: GenerationPlayRoot = semio_framework_pack_json::from_json_str(&fixture["generation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut retirement = root.into_retirement();
        for _ in 0..pause {
            retirement.close_step(1, 1).unwrap();
        }
        assert!(matches!(retirement.close_step(0, 4096).unwrap(), store::SnapshotRetirementStep::Blocked));
        assert!(matches!(retirement.close_step(1, 0).unwrap(), store::SnapshotRetirementStep::Blocked));
        let retirement = std::thread::spawn(move || {
            for _ in 0..100_000 {
                if matches!(retirement.close_step(1, 4096).unwrap(), store::SnapshotRetirementStep::Complete) {
                    break;
                }
            }
            retirement
        })
        .join()
        .unwrap();
        assert!(retirement.terminal_is_empty());
    }
}
//#endregion 🧪️LifecycleGuards
