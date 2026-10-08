//! 🧪️ Laws of the kind-emitting recorder: every editor board edit is stated as the concrete kinds it consists of, one
//! entity at a time, never as a whole board differenced against the document. Each law replays the recorded rows on the
//! document they were recorded on, then their inverse rows last-to-first, and demands the document back; every row also
//! passes the framework's inverse-sum law on the state it was applied to.

use super::Puzzle2dRecorder;
use crate::editor::puzzle2d::{board_snapshot_edges, board_snapshot_nodes, snapshot_target_regions};
use crate::standards::v1::subsets::any::schema::mutations::{delete_node, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;
use protocol::Mutation;
use semio_framework_pack_json::{json, Value};
use std::sync::Arc;

/// 🧱️ Four nodes — `a` (two handles), `b` and `d` (one and two handles) in the middle of `c` — wired `a:v1 → b:v0`, with one
/// target region. `b` is a MIDDLE row, so deleting it also proves position-exact restoration.
fn board() -> Value {
    json!({
        "schema": "board.ports.directed.v1",
        "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },
        "nodes": [
            { "id": "a", "nodeKind": "k", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "A", "handles": [{ "id": "a:v0", "handleKind": "a", "angle": 0.0 }, { "id": "a:v1", "handleKind": "a", "angle": 3.0 }] },
            { "id": "b", "nodeKind": "k", "shape": "circle", "x": 100.0, "y": 0.0, "radius": 24.0, "text": "B", "handles": [{ "id": "b:v0", "handleKind": "a", "angle": 0.0 }] },
            { "id": "c", "nodeKind": "k", "shape": "circle", "x": 200.0, "y": 0.0, "radius": 24.0, "text": "C", "handles": [] },
            { "id": "d", "nodeKind": "k", "shape": "circle", "x": 300.0, "y": 0.0, "radius": 24.0, "text": "D", "handles": [{ "id": "d:v0", "handleKind": "a", "angle": 0.0 }, { "id": "d:v1", "handleKind": "a", "angle": 1.0 }] }
        ],
        "edges": [{ "id": "e1", "source": "a:v1", "target": "b:v0" }],
        "targetRegions": [{ "id": "r1", "x": 0.0, "y": 0.0, "width": 10.0, "height": 10.0, "hidden": false, "locked": false }]
    })
}

fn typed(snapshot: &Value) -> Arc<Puzzle2dSnapshot> {
    Arc::new(semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(snapshot)).expect("typed fixture admits"))
}

fn recorder() -> Puzzle2dRecorder {
    Puzzle2dRecorder::from_typed(typed(&board()))
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|id| id.to_string()).collect()
}

/// ⏪️ Replays `rows` on `base` and then their inverses last-to-first, as the store's undo does; answers the document after the
/// rows and the document after the undo. Every row also satisfies the inverse-sum law on the state it was applied to.
fn replay(base: &Puzzle2dSnapshot, rows: &[Puzzle2dMutation]) -> (Puzzle2dSnapshot, Puzzle2dSnapshot) {
    let mut state = base.clone();
    let mut inverses = Vec::new();
    for row in rows {
        ::semio_framework_async::poll::resolve_ready(protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(row, &state));
        inverses.push(Mutation::<Puzzle2dSnapshot>::inverse(row, &state).expect("every recorded row inverts"));
        state = protocol::apply_diff(Mutation::<Puzzle2dSnapshot>::diff(row, &state).diff(), &state).expect("every recorded row applies");
    }
    let after = state.clone();
    for inverse in inverses.iter().rev() {
        for step in inverse.iter().rev() {
            state = protocol::apply_diff(Mutation::<Puzzle2dSnapshot>::diff(step, &state).diff(), &state).expect("every inverse row applies");
        }
    }
    (after, state)
}

/// ⚖️ LAW: the rows leave exactly the document the recorder holds, and undoing them restores the base.
fn assert_round_trip(base: &Puzzle2dSnapshot, recorder: Puzzle2dRecorder) -> Vec<Puzzle2dMutation> {
    let (document, rows) = recorder.into_parts();
    let (after, restored) = replay(base, &rows);
    assert_eq!(after, *document, "the rows state the document the recorder holds");
    assert_eq!(restored, *base, "the inverse rows, replayed last-to-first, restore the base: {rows:?}");
    rows
}

#[test]
fn add_node_records_one_create_node_labelled_after_its_kind() {
    let base = typed(&board());
    let mut recorder = recorder();
    let id = recorder.add_node(Some("k"), Some(&json!({ "x": 5.0, "y": 6.0 }))).expect("the node is recorded");
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::CreateNode(row)] if row.node.id.eq_str(&id) && row.node.x == 5.0 && row.node.y == 6.0), "{rows:?}");
}

#[test]
fn a_refused_or_idle_kind_is_not_recorded() {
    let mut recorder = recorder();
    assert!(!recorder.record(delete_node("nobody".into())), "a missing target is refused");
    assert!(!recorder.record(crate::standards::v1::subsets::any::schema::mutations::move_node("a".into(), 0.0, 0.0)), "a move onto the same spot changes nothing");
    assert!(recorder.rows().is_empty() && recorder.created_nodes().is_empty());
}

#[test]
fn deleting_entities_records_the_kind_each_one_owns() {
    let base = typed(&board());
    let cases: [(&[&str], fn(&Puzzle2dMutation) -> bool); 4] = [
        (&["c"][..], |row| matches!(row, Puzzle2dMutation::DeleteNode(_))),
        (&["a:v0"][..], |row| matches!(row, Puzzle2dMutation::RemoveNodeHandle(_))),
        (&["e1"][..], |row| matches!(row, Puzzle2dMutation::DisconnectHandles(_))),
        (&["r1"][..], |row| matches!(row, Puzzle2dMutation::DeleteTargetRegion(_))),
    ];
    for (selected, owns) in cases {
        let mut recorder = recorder();
        recorder.delete_entities(&ids(selected));
        let rows = assert_round_trip(&base, recorder);
        assert!(rows.len() == 1 && rows.iter().all(owns), "{selected:?} is one row of its own kind: {rows:?}");
    }
}

#[test]
fn deleting_a_middle_node_severs_its_edges_with_it_and_restores_both_in_place() {
    let base = typed(&board());
    let mut recorder = recorder();
    recorder.delete_entities(&ids(&["b"]));
    assert!(board_snapshot_edges(recorder.value()).is_empty(), "the edge on b's handle left with it");
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::DeleteNode(row)] if row.id.eq_str("b")), "one delete-node states the whole cascade: {rows:?}");
}

#[test]
fn deleting_nodes_handles_edges_and_regions_together_records_each_once() {
    let base = typed(&board());
    let mut recorder = recorder();
    recorder.delete_entities(&ids(&["b", "e1", "d:v1", "r1"]));
    let rows = assert_round_trip(&base, recorder);
    assert_eq!(rows.len(), 3, "the edge was already severed by deleting b: {rows:?}");
}

#[test]
fn flags_are_recorded_on_nodes_handles_and_edges() {
    let base = typed(&board());
    let mut recorder = recorder();
    recorder.set_flag(&ids(&["a", "a:v0", "e1"]), "locked", true);
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::ChangeNodeLocked(_), Puzzle2dMutation::ReplaceNodeHandle(handle), Puzzle2dMutation::ChangeEdgeLocked(_)] if handle.new_handle.locked == Some(true)), "{rows:?}");
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    recorder.set_flag(&ids(&["a", "e1"]), "hidden", true);
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::ChangeNodeVisible(node), Puzzle2dMutation::ChangeEdgeVisible(edge)] if node.new_visible == Some(false) && edge.new_visible == Some(false)), "hidden is stored as its inverse, visible: {rows:?}");
}

#[test]
fn inspector_fields_are_recorded_as_the_kind_that_owns_them() {
    let base = typed(&board());
    let fields: [(&str, Value, fn(&Puzzle2dMutation) -> bool); 5] = [
        ("x", json!(9.0), |row| matches!(row, Puzzle2dMutation::MoveNode(_))),
        ("radius", json!(30.0), |row| matches!(row, Puzzle2dMutation::ReplaceNodeGeometry(_))),
        ("text", json!("Renamed"), |row| matches!(row, Puzzle2dMutation::EditNodeText(_))),
        ("scale", json!(2.0), |row| matches!(row, Puzzle2dMutation::ScaleNode(_))),
        ("locked", json!(true), |row| matches!(row, Puzzle2dMutation::ChangeNodeLocked(_))),
    ];
    for (field, value, owns) in fields {
        let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
        recorder.patch_fields(&ids(&["a"]), field, Some(&value), None);
        let rows = assert_round_trip(&base, recorder);
        assert!(rows.len() == 1 && rows.iter().all(owns), "{field} is one row of its own kind: {rows:?}");
    }
}

#[test]
fn duplicating_nodes_records_clones_and_the_edge_between_them() {
    let base = typed(&board());
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    let clones = recorder.duplicate(&ids(&["a", "b"]));
    assert_eq!(clones.len(), 2);
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::CreateNode(first), Puzzle2dMutation::CreateNode(second), Puzzle2dMutation::ConnectHandles(edge)]
        if first.node.x == 24.0 && second.node.x == 124.0 && first.node.handles.iter().all(|handle| handle.id.as_str().starts_with(first.node.id.as_str()))
        && edge.source.as_str().starts_with(first.node.id.as_str()) && edge.target.as_str().starts_with(second.node.id.as_str())), "{rows:?}");
}

#[test]
fn connecting_and_disconnecting_are_one_row_each() {
    let base = typed(&board());
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    let id = recorder.connect("a:v0", "d:v0", Some("link")).expect("the edge is recorded");
    assert!(recorder.disconnect("e1"));
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::ConnectHandles(edge), Puzzle2dMutation::DisconnectHandles(gone)] if edge.id.eq_str(&id) && gone.id.eq_str("e1")), "{rows:?}");
}

#[test]
fn target_regions_are_painted_moved_resized_flagged_and_deleted_as_their_own_kinds() {
    let base = typed(&board());
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    let painted = recorder.paint_region_cells((13.0, 7.0), (2.0, 3.0), 10.0).expect("the region is painted");
    recorder.relocate_region("r1", &json!({ "position": [1.0, 2.0], "size": [3.0, 4.0] }));
    recorder.set_region_flag(&ids(&["r1"]), "locked", true);
    recorder.delete_regions(&ids(&[painted.as_str()]));
    let rows = assert_round_trip(&base, recorder);
    assert!(
        matches!(rows.as_slice(), [Puzzle2dMutation::CreateTargetRegion(created), Puzzle2dMutation::MoveTargetRegion(_), Puzzle2dMutation::ResizeTargetRegion(_), Puzzle2dMutation::ChangeTargetRegionLocked(_), Puzzle2dMutation::DeleteTargetRegion(_)]
            if created.target_region.x == 10.0 && created.target_region.y == 10.0 && created.target_region.width == 20.0 && created.target_region.height == 30.0),
        "{rows:?}"
    );
}

#[test]
fn a_locked_region_refuses_a_relocation() {
    let mut recorder = recorder();
    recorder.set_region_flag(&ids(&["r1"]), "locked", true);
    let before = recorder.rows().len();
    recorder.relocate_region("r1", &json!({ "position": [5.0, 5.0], "size": [1.0, 1.0] }));
    assert_eq!(recorder.rows().len(), before, "a locked region keeps its pose");
}

#[test]
fn a_brush_placement_records_the_node_and_its_link() {
    let base = typed(&board());
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    let payload = json!({ "nodeId": "placed", "edgeId": "placed-link", "nodeKind": "k", "x": 40.0, "y": 50.0, "sourceHandleId": "d:v0", "targetHandleIndex": 0, "handles": [{ "handleKind": "a", "angle": 0.0 }] });
    assert!(recorder.fold_board_row("brushPlace", &payload));
    assert_eq!(recorder.created_nodes(), ["placed".to_string()]);
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::CreateNode(node), Puzzle2dMutation::ConnectHandles(edge)]
        if node.node.id.eq_str("placed") && node.node.handles.iter().all(|handle| handle.id.eq_str("placed:v0")) && edge.source.eq_str("d:v0") && edge.target.eq_str("placed:v0") && edge.edge_kind.as_ref().is_some_and(|kind| kind.eq_str("link"))), "{rows:?}");
}

#[test]
fn every_board_tool_row_is_recorded_as_its_kinds() {
    let base = typed(&board());
    let rows_and_kinds: [(&str, Value, fn(&Puzzle2dMutation) -> bool); 5] = [
        ("regionCreate", json!({ "x": 5.0, "y": 5.0, "width": 4.0, "height": 4.0 }), |row| matches!(row, Puzzle2dMutation::CreateTargetRegion(_))),
        ("regionResize", json!({ "id": "r1", "x": 1.0, "y": 1.0, "width": 8.0, "height": 8.0 }), |row| matches!(row, Puzzle2dMutation::MoveTargetRegion(_) | Puzzle2dMutation::ResizeTargetRegion(_))),
        ("edgeCreate", json!({ "id": "e2", "source": "a:v0", "target": "d:v0" }), |row| matches!(row, Puzzle2dMutation::ConnectHandles(_))),
        ("edgeDelete", json!({ "id": "e1" }), |row| matches!(row, Puzzle2dMutation::DisconnectHandles(_))),
        ("nodeDelete", json!({ "id": "c" }), |row| matches!(row, Puzzle2dMutation::DeleteNode(_))),
    ];
    for (name, payload, owns) in rows_and_kinds {
        let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
        assert!(recorder.fold_board_row(name, &payload), "{name} is a board-tool row");
        let rows = assert_round_trip(&base, recorder);
        assert!(!rows.is_empty() && rows.iter().all(owns), "{name}: {rows:?}");
    }
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    assert!(!recorder.fold_board_row("hover", &json!({})), "a transient row is not a board-tool row");
    assert!(recorder.rows().is_empty());
    recorder.fold_board_row("regionCreate", &json!({ "x": 5.0, "y": 5.0, "width": 0.0, "height": 4.0 }));
    assert!(recorder.rows().is_empty(), "an empty area paints nothing");
}

#[test]
fn laid_out_poses_are_recorded_as_node_moves_and_unmoved_nodes_state_nothing() {
    let base = typed(&board());
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    let node=|id:&str,x,y|semio_framework_os_infinite::board::ports::directed::schema::snapshot::BoardNodeSnapshot{id:id.into(),x:Some(x),y:Some(y),..Default::default()};recorder.place_nodes(&[node("a",0.0,0.0),node("b",101.0,2.0),node("ghost",1.0,1.0)]);
    let rows = assert_round_trip(&base, recorder);
    assert!(matches!(rows.as_slice(), [Puzzle2dMutation::MoveNode(row)] if row.id.eq_str("b") && row.new_x == 101.0 && row.new_y == 2.0), "{rows:?}");
}

#[test]
fn cutting_nodes_records_their_removal_and_one_undo_restores_the_board() {
    let base = typed(&board());
    let mut recorder = Puzzle2dRecorder::from_typed(base.clone());
    recorder.cut(&ids(&["a", "b"]));
    assert_eq!(board_snapshot_nodes(recorder.value()).len(), 2);
    assert!(board_snapshot_edges(recorder.value()).is_empty());
    assert_eq!(snapshot_target_regions(recorder.value()).len(), 1, "a cut leaves the regions alone");
    let rows = assert_round_trip(&base, recorder);
    assert!(rows.iter().all(|row| matches!(row, Puzzle2dMutation::DeleteNode(_))), "{rows:?}");
}
