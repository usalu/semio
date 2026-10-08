//! 🧪️ Every board-editing command emits the concrete kinds of its user action — never a board differenced against the
//! document — and undoing the emitted rows (inverse rows replayed last-to-first) restores the board. Driven through
//! `puzzle2d_dispatch_emit`, the one pipeline every command runs; whole-document import leaves through the load path.

use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;
use protocol::Mutation;

fn board() -> Value {
    json!({
        "schema": "board.ports.directed.v1",
        "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },
        "nodes": [
            { "id": "a", "nodeKind": "k", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "A", "handles": [{ "id": "a:v0", "handleKind": "a", "angle": 0.0 }, { "id": "a:v1", "handleKind": "a", "angle": 3.0 }] },
            { "id": "b", "nodeKind": "k", "shape": "circle", "x": 100.0, "y": 0.0, "radius": 24.0, "text": "B", "handles": [{ "id": "b:v0", "handleKind": "a", "angle": 0.0 }] },
            { "id": "c", "nodeKind": "k", "shape": "circle", "x": 200.0, "y": 0.0, "radius": 24.0, "text": "C", "handles": [] },
            { "id": "d", "nodeKind": "k", "shape": "circle", "x": 300.0, "y": 0.0, "radius": 24.0, "text": "D", "handles": [{ "id": "d:v0", "handleKind": "a", "angle": 0.0 }] }
        ],
        "edges": [{ "id": "e1", "source": "a:v1", "target": "b:v0" }],
        "targetRegions": [{ "id": "r1", "x": 0.0, "y": 0.0, "width": 10.0, "height": 10.0, "hidden": false, "locked": false }]
    })
}

fn snapshot_of(board: Value) -> Puzzle2dPlaySnapshot {
    Puzzle2dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&board)).expect("typed fixture admits"))
}

fn emit_for(snapshot: &Puzzle2dPlaySnapshot, action: &str, args: Value, selected: &[&str]) -> Emit<Puzzle2dMutation, Puzzle2dConfigMutation> {
    let command = Puzzle2dCommand::from_action(action, Some(args), Some(overview::WINDOW_KIND_ID.to_string()));
    let selection = protocol::DomainSelection { granularity: PUZZLE2D_GRANULARITY_NODE.into(), ids: selected.iter().map(|id| id.to_string()).collect(), anchor_id: None };
    let view = window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID);
    puzzle2d_dispatch_emit(&command, snapshot, &Puzzle2dConfig::default(), &Puzzle2dWindowConfig::default(), &Puzzle2dWindowTransient::default(), overview::WINDOW_KIND_ID, Some(&view), select_utility::UTILITY_ID, &selection, "seed", "rev-1", &semio_framework_plugin::app::GestureSlot::detached(), None)
        .unwrap_or_else(|fault| panic!("{action} emits: {fault:?}"))
        .0
}

fn apply(state: &crate::Puzzle2dSnapshot, row: &Puzzle2dMutation) -> crate::Puzzle2dSnapshot {
    protocol::apply_diff(Mutation::<crate::Puzzle2dSnapshot>::diff(row, state).diff(), state).expect("the row applies")
}

fn assert_undo_restores(base: &Puzzle2dPlaySnapshot, rows: &[Puzzle2dMutation]) {
    let mut state = base.typed().clone();
    let mut inverses = Vec::new();
    for row in rows {
        inverses.push(Mutation::<crate::Puzzle2dSnapshot>::inverse(row, &state).expect("the row inverts"));
        state = apply(&state, row);
    }
    for inverse in inverses.iter().rev() {
        for step in inverse.iter().rev() {
            state = apply(&state, step);
        }
    }
    assert_eq!(&state, base.typed(), "the inverse rows, replayed last-to-first, restore the board: {rows:?}");
}

fn rows_of(action: &str, args: Value, selected: &[&str]) -> Vec<Puzzle2dMutation> {
    let snapshot = snapshot_of(board());
    let emit = emit_for(&snapshot, action, args, selected);
    assert_undo_restores(&snapshot, &emit.artifact_mutations);
    emit.artifact_mutations
}

#[test]
fn add_node_emits_one_create_node() {
    let rows = rows_of("addNode", json!({ "kind": "k", "x": 5.0, "y": 6.0 }), &[]);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::CreateNode(row)] if row.node.x == 5.0 && row.node.y == 6.0), "{rows:?}");
}

#[test]
fn delete_selection_emits_the_kind_each_selected_entity_owns() {
    let rows = rows_of("deleteSelection", json!({}), &["b", "d:v0", "r1"]);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::DeleteNode(node), Puzzle2dMutation::RemoveNodeHandle(handle), Puzzle2dMutation::DeleteTargetRegion(_)] if node.id.eq_str("b") && handle.handle_id.eq_str("d:v0")), "{rows:?}");
    let edge = rows_of("deleteSelection", json!({}), &["e1"]);
    assert!(matches!(edge.as_slice(), [Puzzle2dMutation::DisconnectHandles(_)]), "{edge:?}");
}

#[test]
fn duplicate_selection_emits_clones_and_the_edge_between_them() {
    let rows = rows_of("duplicateSelection", json!({}), &["a", "b"]);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::CreateNode(_), Puzzle2dMutation::CreateNode(_), Puzzle2dMutation::ConnectHandles(_)]), "{rows:?}");
}

#[test]
fn selection_flags_emit_flag_kinds() {
    let rows = rows_of("setSelectionFlag", json!({ "flag": "locked", "value": true }), &["a", "a:v0", "e1"]);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::ChangeNodeLocked(_), Puzzle2dMutation::ReplaceNodeHandle(_), Puzzle2dMutation::ChangeEdgeLocked(_)]), "{rows:?}");
    let hidden = rows_of("setSelectionHidden", json!({ "hidden": true }), &["a"]);
    assert!(matches!(hidden.as_slice(), [Puzzle2dMutation::ChangeNodeVisible(row)] if row.new_visible == Some(false)), "{hidden:?}");
}

#[test]
fn inspector_patches_emit_the_kind_that_owns_the_field() {
    let moved = rows_of("patchInspectorNodes", json!({ "ids": ["a"], "field": "x", "value": 9.0 }), &[]);
    assert!(matches!(moved.as_slice(), [Puzzle2dMutation::MoveNode(row)] if row.new_x == 9.0), "{moved:?}");
    let sized = rows_of("patchInspectorNodes", json!({ "ids": ["a"], "field": "radius", "value": 30.0 }), &[]);
    assert!(matches!(sized.as_slice(), [Puzzle2dMutation::ReplaceNodeGeometry(_)]), "{sized:?}");
    let turned = rows_of("patchInspectorNodes", json!({ "ids": ["a:v1"], "field": "angle", "delta": 0.5 }), &[]);
    assert!(matches!(turned.as_slice(), [Puzzle2dMutation::ReplaceNodeHandle(row)] if row.new_handle.angle == 3.5), "{turned:?}");
}

#[test]
fn edges_are_connected_and_disconnected_as_their_own_kinds() {
    let connected = rows_of("createEdge", json!({ "source": "a:v0", "target": "d:v0" }), &[]);
    assert!(matches!(connected.as_slice(), [Puzzle2dMutation::ConnectHandles(row)] if row.source.eq_str("a:v0") && row.target.eq_str("d:v0")), "{connected:?}");
    let cut = rows_of("deleteEdge", json!({ "id": "e1" }), &[]);
    assert!(matches!(cut.as_slice(), [Puzzle2dMutation::DisconnectHandles(row)] if row.id.eq_str("e1")), "{cut:?}");
}

#[test]
fn proximity_connect_emits_a_connection_stating_its_tolerance() {
    let facing = json!({
        "schema": "board.ports.directed.v1",
        "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },
        "nodes": [
            { "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "left:v0", "handleKind": "a", "angle": 0.0 }] },
            { "id": "right", "shape": "circle", "x": 52.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "right:v0", "handleKind": "a", "angle": std::f64::consts::PI }] }
        ],
        "edges": []
    });
    let snapshot = snapshot_of(facing);
    let emit = emit_for(&snapshot, "proximityConnect", json!({ "nodeId": "right", "radius": 12.0 }), &[]);
    assert!(matches!(emit.artifact_mutations.as_slice(), [Puzzle2dMutation::ConnectHandles(row)] if row.tolerance == Some(12.0) && row.source.eq_str("left:v0")), "{:?}", emit.artifact_mutations);
    assert_undo_restores(&snapshot, &emit.artifact_mutations);
}

#[test]
fn target_region_commands_emit_region_kinds() {
    let painted = rows_of("addTargetRegion", json!({ "origin": [20.0, 20.0], "size": [2.0, 2.0] }), &[]);
    assert!(matches!(painted.as_slice(), [Puzzle2dMutation::CreateTargetRegion(_)]), "{painted:?}");
    let deleted = rows_of("deleteTargetRegion", json!({ "id": "r1" }), &[]);
    assert!(matches!(deleted.as_slice(), [Puzzle2dMutation::DeleteTargetRegion(_)]), "{deleted:?}");
    let relocated = rows_of("relocateTargetRegion", json!({ "regionId": "r1", "after": { "position": [1.0, 2.0], "size": [3.0, 4.0] } }), &[]);
    assert!(matches!(relocated.as_slice(), [Puzzle2dMutation::MoveTargetRegion(_), Puzzle2dMutation::ResizeTargetRegion(_)]), "{relocated:?}");
    let flagged = rows_of("setTargetRegionFlag", json!({ "id": "r1", "flag": "locked", "value": true }), &[]);
    assert!(matches!(flagged.as_slice(), [Puzzle2dMutation::ChangeTargetRegionLocked(_)]), "{flagged:?}");
}

#[test]
fn force_layout_emits_only_node_moves() {
    let rows = rows_of("forceLayout", json!({}), &[]);
    assert!(!rows.is_empty() && rows.iter().all(|row| matches!(row, Puzzle2dMutation::MoveNode(_))), "{rows:?}");
}

#[test]
fn board_tool_rows_emit_their_kinds_as_one_transaction() {
    let cases = [
        (json!([{ "name": "nodeDelete", "payload": { "id": "c" } }]), "DeleteNode"),
        (json!([{ "name": "edgeDelete", "payload": { "id": "e1" } }]), "DisconnectHandles"),
        (json!([{ "name": "regionCreate", "payload": { "x": 5.0, "y": 5.0, "width": 4.0, "height": 4.0 } }]), "CreateTargetRegion"),
        (json!([{ "name": "edgeCreate", "payload": { "id": "e2", "source": "a:v0", "target": "d:v0" } }]), "ConnectHandles"),
    ];
    for (events, kind) in cases {
        let snapshot = snapshot_of(board());
        let emit = emit_for(&snapshot, "applyBoardEvents", json!({ "eventsJson": events.to_string() }), &[]);
        assert!(emit.transaction.is_some(), "{kind}: one tool transaction");
        assert!(emit.artifact_mutations.len() == 1 && format!("{:?}", emit.artifact_mutations[0]).starts_with(kind), "{kind}: {:?}", emit.artifact_mutations);
        assert_undo_restores(&snapshot, &emit.artifact_mutations);
    }
}

#[test]
fn importing_a_snapshot_loads_the_whole_document_and_states_no_mutation() {
    let snapshot = snapshot_of(json!({ "schema": "board.ports.directed.v1", "nodes": [], "edges": [] }));
    let emit = emit_for(&snapshot, "importSnapshot", json!({ "payload": board() }), &[]);
    assert!(emit.artifact_mutations.is_empty(), "an import is no mutation: {:?}", emit.artifact_mutations);
    let loads: Vec<&Vec<u8>> = emit.effects.iter().filter_map(|effect| match effect {
        Effect::LoadDocument { pack, .. } => Some(pack),
        _ => None,
    }).collect();
    assert_eq!(loads.len(), 1, "one whole-document load: {:?}", emit.effects);
    let loaded = <crate::Puzzle2dSnapshot as store::ArtifactPack>::decode_pack(loads[0]).expect("the loaded pack decodes");
    let expected: crate::Puzzle2dSnapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&board())).expect("board admits");
    assert_eq!(loaded, expected, "the load carries exactly the imported document");
}

#[test]
fn importing_a_malformed_snapshot_raises_one_notice_and_loads_nothing() {
    let snapshot = snapshot_of(board());
    let emit = emit_for(&snapshot, "importSnapshot", json!({ "payload": { "schema": "board.ports.directed.v1", "nodes": [{ "id": "n", "x": "left" }] } }), &[]);
    assert!(emit.artifact_mutations.is_empty());
    assert!(!emit.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })), "{:?}", emit.effects);
    assert_eq!(emit.effects.iter().filter(|effect| matches!(effect, Effect::Notify { .. })).count(), 1);
}

#[test]
fn cut_and_paste_emit_node_kinds() {
    let snapshot = snapshot_of(board());
    let cut = puzzle2d_cut_operations_from(&snapshot, &["a".to_string(), "b".to_string()]);
    assert!(cut.len() == 2 && cut.iter().all(|row| matches!(row, Puzzle2dMutation::DeleteNode(_))), "{cut:?}");
    assert_undo_restores(&snapshot, &cut);
    let fragment = puzzle2d_copy_fragment_from(snapshot.value(), &["a".to_string(), "b".to_string()]).expect("copy");
    let (pasted, ids) = puzzle2d_paste_operations_on(&snapshot, &fragment, &PastePlacement::default()).expect("paste");
    assert_eq!(ids.len(), 2);
    assert!(matches!(pasted.as_slice(), [Puzzle2dMutation::CreateNode(_), Puzzle2dMutation::CreateNode(_), Puzzle2dMutation::ConnectHandles(_)]), "{pasted:?}");
    assert_undo_restores(&snapshot, &pasted);
}
