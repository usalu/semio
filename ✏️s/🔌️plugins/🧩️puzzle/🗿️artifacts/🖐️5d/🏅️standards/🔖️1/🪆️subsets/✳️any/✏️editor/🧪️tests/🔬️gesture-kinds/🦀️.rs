//! 🧭️ Host gesture protocol laws for the 5d editor: gestures (verbs with ids and values) become the concrete mutation
//! kinds they consist of, built from the typed base alone; every law replays the kinds forward through the central
//! applier and the recorded inverse rows last-to-first, and requires the base back.

use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{change_part_2d_hidden, change_part_2d_locked, delete_part, disconnect_grips, remove_part_grip};

fn base() -> Puzzle5dSnapshot {
    let part = |id: &str, x: f64| serde_json::json!({ "id": id, "2d": { "x": x, "y": 0.0 }, "3d": { "origin": [x, 0.0, 0.0] }, "grips": [{ "id": "g0" }] });
    let value = serde_json::json!({
        "schema": crate::PUZZLE_5D_SCHEMA, "domain": "architecture", "meta": { "description": "" },
        "parts": [part("a", 0.0), part("b", 1.0), part("c", 2.0)],
        "fasteners": [{ "id": "f1", "source": "a:g0", "target": "b:g0" }, { "id": "f2", "source": "b:g0", "target": "c:g0" }],
    });
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture")
}

fn replay(before: &Puzzle5dSnapshot, operations: &[Puzzle5dMutation]) -> (Puzzle5dSnapshot, Puzzle5dSnapshot) {
    let mut state = before.clone();
    let mut inverses = Vec::new();
    for operation in operations {
        inverses.extend(protocol::Mutation::<Puzzle5dSnapshot>::inverse(operation, &state).expect("concrete inverse"));
        state = protocol::apply_diff(protocol::Mutation::<Puzzle5dSnapshot>::diff(operation, &state).diff(), &state).expect("central apply");
    }
    let after = state.clone();
    for inverse in inverses.iter().rev() {
        state = protocol::apply_diff(protocol::Mutation::<Puzzle5dSnapshot>::diff(inverse, &state).diff(), &state).expect("central inverse apply");
    }
    (after, state)
}

#[test]
fn a_delete_gesture_on_a_part_is_one_delete_part_and_its_fasteners_ride_the_cascade() {
    let before = base();
    let operations = delete_selection::delete_selection_mutations(&before, &["b".to_string()], &[], &["f1".to_string()]);
    assert_eq!(operations, vec![delete_part("b".into())], "f1 is severed by deleting b, so it names no row of its own");
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.parts.len(), 2);
    assert!(after.fasteners.is_empty());
    assert_eq!(restored, before);
}

#[test]
fn a_delete_gesture_on_a_fastener_or_a_grip_is_its_own_concrete_kind() {
    let before = base();
    let fastener = delete_selection::delete_selection_mutations(&before, &[], &[], &["f2".to_string()]);
    assert_eq!(fastener, vec![disconnect_grips("f2".into())]);
    assert_eq!(replay(&before, &fastener).1, before);
    let grip = delete_selection::delete_selection_mutations(&before, &[], &["c:g0".to_string()], &[]);
    assert_eq!(grip, vec![remove_part_grip("c".into(), "g0".into())]);
    let (after, restored) = replay(&before, &grip);
    assert!(after.parts[2].grips.is_empty());
    assert_eq!(after.fasteners.len(), 1, "the removed grip's fastener is severed with it");
    assert_eq!(restored, before);
}

#[test]
fn a_flag_gesture_emits_one_row_per_part_whose_flag_differs() {
    let before = base();
    let hide = set_selection_flag::selection_flag_mutations(&before, PUZZLE5D_GRANULARITY_PART, &["a".to_string(), "b".to_string()], "hidden", true);
    assert_eq!(hide, vec![change_part_2d_hidden("a".into(), Some(true)), change_part_2d_hidden("b".into(), Some(true))]);
    let (after, restored) = replay(&before, &hide);
    assert_eq!(after.parts[0].part_2d.hidden, Some(true));
    assert_eq!(restored, before);
    assert!(set_selection_flag::selection_flag_mutations(&after, PUZZLE5D_GRANULARITY_PART, &["a".to_string()], "hidden", true).is_empty());
    assert_eq!(set_selection_flag::selection_flag_mutations(&before, PUZZLE5D_GRANULARITY_PART, &["c".to_string()], "locked", true), vec![change_part_2d_locked("c".into(), Some(true))]);
    assert!(set_selection_flag::selection_flag_mutations(&before, "fastener", &["f1".to_string()], "hidden", true).is_empty());
}

#[test]
fn a_cut_gesture_disconnects_before_it_deletes_and_undo_restores_the_document() {
    let before = base();
    let snapshot = Puzzle5dPlaySnapshot::new(before.clone());
    let operations = puzzle5d_cut_operations(&snapshot, &["a".to_string(), "b".to_string()], &[]);
    assert_eq!(operations, vec![disconnect_grips("f1".into()), delete_part("a".into()), delete_part("b".into())]);
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.parts.len(), 1);
    assert_eq!(restored, before);
}

#[test]
fn a_paste_gesture_is_a_create_part_per_clone_then_a_connect_grips_per_fastener() {
    let before = base();
    let document = puzzle5d_document_from_snapshot(&before).expect("host projection");
    let (parts, fasteners) = copy_selection_local(&document, &["a".to_string(), "b".to_string()], &[]);
    let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &parts, &fasteners, (48.0, 24.0));
    let operations = puzzle5d_paste_mutations(fresh_parts, fresh_fasteners).expect("materialized paste");
    assert_eq!(operations.len(), 3);
    assert!(matches!(operations[0], Puzzle5dMutation::CreatePart(_)) && matches!(operations[1], Puzzle5dMutation::CreatePart(_)) && matches!(operations[2], Puzzle5dMutation::ConnectGrips(_)), "{operations:?}");
    let (after, restored) = replay(&before, &operations);
    assert_eq!((after.parts.len(), after.fasteners.len()), (5, 3));
    assert_eq!(restored, before);
}

#[test]
fn a_painted_volume_is_one_create_target_volume_and_a_flag_gesture_on_it_is_one_change_row() {
    let before = base();
    let create = commands::add_target_volume::add_target_volume_mutation("target-volume-1".into(), [1.1, 2.0, 2.9], 1.0, [2, 2, 2]);
    let (after, restored) = replay(&before, std::slice::from_ref(&create));
    assert_eq!(after.target_volumes.len(), 1);
    assert_eq!(restored, before);
    let hide = crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden("target-volume-1".into(), true);
    let (hidden, restored) = replay(&after, &[hide]);
    assert!(hidden.target_volumes[0].hidden);
    assert_eq!(restored, after);
}
