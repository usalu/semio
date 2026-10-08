//! 🧭️ Host gesture protocol laws: the renderer and the panels report GESTURES (verbs with ids and values), never a scene,
//! and every gesture the plugin turns into a document edit is the concrete mutation kinds it consists of. Each law
//! builds the kinds from a typed base alone, replays them forward through the central applier and the recorded
//! inverse rows last-to-first, and requires the base back.

use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{change_object_hidden, change_object_locked, create_object, delete_object, disconnect_vortices};

fn vortex(id: &str) -> Puzzle3dVortex {
    Puzzle3dVortex { id: id.into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }
}

fn object(id: &str, origin: [f64; 3]) -> Puzzle3dObject {
    Puzzle3dObject { id: id.into(), label: None, object_kind: Some("Object".into()), origin, orientation: None, scale: None, mesh_url: None, vortices: vec![vortex("v0")], hidden: false, locked: false }
}

fn base() -> Puzzle3dSnapshot {
    let mut scene = empty_scene_snapshot();
    scene.objects = vec![object("a", [0.0, 0.0, 0.0]), object("b", [1.0, 0.0, 0.0]), object("c", [2.0, 0.0, 0.0])];
    scene.attractions = vec![
        Puzzle3dAttraction { id: "t1".into(), attracting: "a:v0".into(), attracted: "b:v0".into(), ..Default::default() },
        Puzzle3dAttraction { id: "t2".into(), attracting: "b:v0".into(), attracted: "c:v0".into(), ..Default::default() },
    ];
    puzzle3d_snapshot_from_host_snapshot(&scene).expect("fixture admits")
}

fn replay(before: &Puzzle3dSnapshot, operations: &[Puzzle3dMutation]) -> (Puzzle3dSnapshot, Puzzle3dSnapshot) {
    let mut state = before.clone();
    let mut inverses = Vec::new();
    for operation in operations {
        inverses.extend(protocol::Mutation::<Puzzle3dSnapshot>::inverse(operation, &state).expect("concrete inverse"));
        state = protocol::apply_diff(protocol::Mutation::<Puzzle3dSnapshot>::diff(operation, &state).diff(), &state).expect("central apply");
    }
    let after = state.clone();
    for inverse in inverses.iter().rev() {
        state = protocol::apply_diff(protocol::Mutation::<Puzzle3dSnapshot>::diff(inverse, &state).diff(), &state).expect("central inverse apply");
    }
    (after, state)
}

#[test]
fn a_delete_gesture_on_an_object_is_one_delete_object_and_undo_restores_the_document() {
    let before = base();
    let operations = delete_selection::delete_selection_mutations(&before, &["b".to_string()], &HashSet::new(), &[], &[], &[]);
    assert_eq!(operations, vec![delete_object("b".into())]);
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.objects.len(), 2);
    assert!(after.attractions.is_empty(), "the object's delete severs both attractions it carries");
    assert_eq!(restored, before);
}

#[test]
fn a_delete_gesture_on_an_attraction_is_one_disconnect_and_a_cascaded_attraction_adds_no_second_row() {
    let before = base();
    let alone = delete_selection::delete_selection_mutations(&before, &[], &HashSet::new(), &["t1".to_string()], &[], &[]);
    assert_eq!(alone, vec![disconnect_vortices("t1".into())]);
    let (after, restored) = replay(&before, &alone);
    assert_eq!(after.attractions.len(), 1);
    assert_eq!(restored, before);
    let with_owner = delete_selection::delete_selection_mutations(&before, &["a".to_string()], &HashSet::new(), &["t1".to_string()], &[], &[]);
    assert_eq!(with_owner, vec![delete_object("a".into())], "t1 is severed by deleting a, so it names no row of its own");
    let (_, restored) = replay(&before, &with_owner);
    assert_eq!(restored, before);
}

#[test]
fn a_delete_gesture_on_a_vortex_removes_only_that_vortex_of_a_surviving_object() {
    let before = base();
    let vortices: HashSet<String> = [puzzle3d_vortex_full_id("c", "v0")].into_iter().collect();
    let operations = delete_selection::delete_selection_mutations(&before, &[], &vortices, &[], &[], &[]);
    assert_eq!(operations, vec![crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex("c".into(), "v0".into())]);
    let (after, restored) = replay(&before, &operations);
    assert!(after.objects[2].vortices.is_empty());
    assert_eq!(after.attractions.len(), 1, "the vortex's attraction is severed with it");
    assert_eq!(restored, before);
}

#[test]
fn a_flag_gesture_emits_one_row_per_entity_whose_flag_differs_and_none_when_it_already_holds() {
    let before = base();
    let hide = set_selection_flag::selection_flag_mutations(&before, "object", &["a".to_string(), "c".to_string()], "hidden", true);
    assert_eq!(hide, vec![change_object_hidden("a".into(), true), change_object_hidden("c".into(), true)]);
    let (after, restored) = replay(&before, &hide);
    assert!(after.objects[0].hidden && !after.objects[1].hidden && after.objects[2].hidden);
    assert_eq!(restored, before);
    assert!(set_selection_flag::selection_flag_mutations(&after, "object", &["a".to_string()], "hidden", true).is_empty());
    assert_eq!(set_selection_flag::selection_flag_mutations(&before, "object", &["b".to_string()], "locked", true), vec![change_object_locked("b".into(), true)]);
}

#[test]
fn a_vortex_flag_gesture_is_one_replace_object_vortex_carrying_the_base_vortex_with_the_flag() {
    let before = base();
    let operations = set_selection_flag::selection_flag_mutations(&before, "vortex", &[puzzle3d_vortex_full_id("b", "v0")], "locked", true);
    assert_eq!(operations.len(), 1);
    let (after, restored) = replay(&before, &operations);
    assert!(after.objects[1].vortices[0].locked);
    assert_eq!(restored, before);
}

#[test]
fn a_cut_gesture_is_the_delete_kinds_of_the_selected_objects() {
    let before = base();
    let marks = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected: vec!["a".into(), "c".into()], hovered: Vec::new(), referenced: Vec::new() };
    let operations = puzzle3d_cut_operations_from(&before, &marks);
    assert_eq!(operations, vec![delete_object("a".into()), delete_object("c".into())]);
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.objects.len(), 1);
    assert_eq!(restored, before);
}

#[test]
fn a_paste_gesture_is_one_create_object_per_clone_with_a_fresh_id() {
    let before = base();
    let marks = Puzzle3dInteractionSnapshot { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), selected: vec!["a".into()], hovered: Vec::new(), referenced: Vec::new() };
    let host = puzzle3d_scene_snapshot_from_document(&before);
    let fragment = puzzle3d_copy_fragment_from(&host, puzzle3d_selected_objects_from(&marks, &host)).expect("copy fragment");
    let operations = puzzle3d_paste_operations(&fragment, &PastePlacement::default()).expect("paste");
    assert_eq!(operations.len(), 1);
    assert!(matches!(&operations[0], Puzzle3dMutation::CreateObject(create) if create.object.id != "a" && create.object.origin == [0.5, 0.5, 0.0]), "{operations:?}");
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.objects.len(), 4);
    assert_eq!(restored, before);
}

#[test]
fn a_duplicate_clone_is_a_create_object_and_a_painted_volume_is_a_create_target_volume() {
    let before = base();
    let clone = create_object(crate::Puzzle3dObject { id: "a-copy".into(), origin: [0.5, 0.5, 0.0], ..before.objects[0].clone() }, None);
    let volume = add_target_volume::add_target_volume_mutation([1.1, 2.0, 2.9], 1.0, [2, 2, 2]);
    let (after, restored) = replay(&before, &[clone, volume]);
    assert_eq!(after.objects.len(), 4);
    assert_eq!(after.target_volumes.len(), 1);
    assert_eq!(after.target_volumes[0].origin, [1.0, 2.0, 3.0]);
    assert_eq!(restored, before);
}
