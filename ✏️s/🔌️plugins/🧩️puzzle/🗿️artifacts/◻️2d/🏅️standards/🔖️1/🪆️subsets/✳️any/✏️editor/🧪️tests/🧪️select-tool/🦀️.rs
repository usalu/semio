//! 🧪️ The select tool machine on its own: gesture records decode, a record yields its parametric leaf plus the
//! connections its drop lands with minted ids, the commit is ONE transaction under a deterministic ref,
//! all-locked, identity, missing and empty requests leave zero trace, a streamed gesture is the window's ONE gesture on
//! the framework runner (resumed by stable ids, committed as ONE transaction or dropped with zero trace), and a committed
//! leaf states its inputs at their declared precision.

use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{ConnectHandles,DragSelection};

use protocol::Mutation as _;
use semio_framework_tool_machine::{drive_chart_gesture, ChartGesture, GestureDrive, GestureState, GestureTool, ToolAbortReason, ToolRefusal};
use serde_json::json;

/// 🧱️ Two facing circle nodes 1000 apart whose `v0` handles are compatible, a locked node, and one free region.
fn board() -> Puzzle2dSnapshot {
    let snapshot = json!({
        "schema": "board.ports.directed.v1",
        "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },
        "nodes": [
            { "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "left:v0", "handleKind": "a", "angle": 0.0 }] },
            { "id": "right", "shape": "circle", "x": 1000.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "right:v0", "handleKind": "a", "angle": std::f64::consts::PI }] },
            { "id": "locked", "shape": "circle", "x": 500.0, "y": 500.0, "radius": 24.0, "locked": true, "handles": [] }
        ],
        "edges": [],
        "targetRegions": [{ "id": "region-1", "x": 10.0, "y": 10.0, "width": 20.0, "height": 20.0, "hidden": false, "locked": false }]
    });
    semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&snapshot)).expect("typed board")
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn hlc0() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }
}

fn commit(seed: &str, base: &Puzzle2dSnapshot, radius: f64, records: Vec<Puzzle2dSelectionRecord>) -> Option<(protocol::TransactionRef, Vec<Puzzle2dMutation>)> {
    puzzle2d_select_tool_commit("select", seed, hlc0(), SelectToolRequest { base: Arc::new(base.clone()), proximity_radius: radius, records })
}

fn request(base: &Puzzle2dSnapshot, records: Vec<Puzzle2dSelectionRecord>) -> SelectToolRequest {
    SelectToolRequest { base: Arc::new(base.clone()), proximity_radius: 12.0, records }
}

#[test]
fn a_board_gesture_record_decodes_every_kind_and_refuses_malformed_ones() {
    let drag = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "gesture-1", "kind": "drag", "targets": ["left"], "dx": 8.0, "dy": -4.0, "proximity": [{ "source": "left:v0", "target": "right:v0" }] })).expect("drag decodes");
    assert_eq!(drag, Puzzle2dSelectionRecord { targets: ids(&["left"]), motion: Puzzle2dSelectionMotion::Drag { dx: 8.0, dy: -4.0 }, proximity: vec![("left:v0".into(), "right:v0".into())], connect: true });
    let rotate = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "gesture-2", "kind": "rotate", "targets": ["left"], "pivotX": 1.0, "pivotY": 2.0, "angle": 0.5, "proximity": [] })).expect("rotate decodes");
    assert_eq!((rotate.motion, rotate.connect), (Puzzle2dSelectionMotion::Rotate { pivot_x: 1.0, pivot_y: 2.0, angle: 0.5 }, false), "a rotation never drops");
    let scale = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "gesture-3", "kind": "scale", "targets": ["left"], "pivotX": 1.0, "pivotY": 2.0, "factor": 2.0, "proximity": [] })).expect("scale decodes");
    assert_eq!(scale.motion, Puzzle2dSelectionMotion::Scale { pivot_x: 1.0, pivot_y: 2.0, factor: 2.0 });
    for malformed in [json!({ "kind": "drag", "targets": ["left"], "dx": 1.0 }), json!({ "kind": "spin", "targets": ["left"] }), json!({ "kind": "drag", "dx": 1.0, "dy": 1.0 }), json!({ "kind": "drag", "targets": ["left"], "dx": "far", "dy": 1.0 })] {
        assert_eq!(Puzzle2dSelectionRecord::from_gesture(&malformed), None, "{malformed} must not decode");
    }
}

#[test]
fn a_drag_record_yields_its_leaf_then_the_connection_its_drop_lands() {
    let base = board();
    let yields = puzzle2d_selection_yields(&base, &[Puzzle2dSelectionRecord::drag(ids(&["right"]), -948.0, 0.0)], 12.0);
    assert_eq!(yields.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), vec!["selection:0", "connect:edge-left:v0-right:v0"]);
    assert_eq!(yields[0].1, Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: -948.0, dy: 0.0 }), "the leaf records the literal targets and the offset");
    let Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, tolerance, .. }) = &yields[1].1 else { panic!("the drop lands a connection: {:?}", yields[1].1) };
    assert_eq!((id.as_str(), source.as_str(), target.as_str()), ("edge-left:v0-right:v0", "left:v0", "right:v0"), "the stationary peer is the source");
    assert_eq!(*tolerance, Some(12.0), "the connection states the radius its search found it within");
    let far = puzzle2d_selection_yields(&base, &[Puzzle2dSelectionRecord::drag(ids(&["right"]), -500.0, 0.0)], 12.0);
    assert_eq!(far.len(), 1, "a drop outside the radius lands no connection");
    let quiet = puzzle2d_selection_yields(&base, &[Puzzle2dSelectionRecord { connect: false, ..Puzzle2dSelectionRecord::drag(ids(&["right"]), -948.0, 0.0) }], 12.0);
    assert_eq!(quiet.len(), 1, "a record that does not drop never searches");
}

#[test]
fn recorded_pairs_connect_as_recorded_and_minted_ids_stay_unique() {
    let mut base = board();
    base.edges.push(crate::Puzzle2dEdge { id: "edge-right:v0-left:v0".into(), source: "ghost:a".into(), target: "ghost:b".into(), ..Default::default() });
    let record = Puzzle2dSelectionRecord { targets: ids(&["right"]), motion: Puzzle2dSelectionMotion::Drag { dx: -10.0, dy: 0.0 }, proximity: vec![("right:v0".into(), "left:v0".into()), ("right:v0".into(), "missing:v9".into())], connect: false };
    let yields = puzzle2d_selection_yields(&base, &[record], 0.0);
    assert_eq!(yields.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), vec!["selection:0", "connect:edge-right:v0-left:v0-2"], "the recorded pair connects as recorded, past the id the document already holds; a pair naming a missing handle is dropped");
    let Puzzle2dMutation::ConnectHandles(ConnectHandles { tolerance, .. }) = &yields[1].1 else { panic!("the recorded pair lands a connection: {:?}", yields[1].1) };
    let mut dropped = base.clone();
    apply_puzzle2d_mutation(&mut dropped, &yields[0].1).expect("the drag applies");
    assert_eq!(*tolerance, crate::standards::v1::subsets::any::schema::mutations::puzzle2d_handle_distance(&dropped, "right:v0", "left:v0"), "a pair the board recorded from farther than the radius states its recorded distance");
    assert!(yields[1].1.diff(&dropped).messages().is_empty(), "replayed where it was recorded, the connection reports nothing");
    assert_eq!(puzzle2d_minted_edge_id(&board(), "a", "b"), "edge-a-b");
}

#[test]
fn the_commit_is_one_transaction_under_a_deterministic_ref() {
    let base = board();
    let records = vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -948.0, 0.0)];
    let (transaction, mutations) = commit("seed-1", &base, 12.0, records.clone()).expect("a movable drag commits");
    assert_eq!(transaction, protocol::TransactionRef::mint(&protocol::ActorId("seed-1".into()), &hlc0(), "s.puzzle.puzzle2d@1/*#editor#select"));
    assert_eq!(transaction.tool, "s.puzzle.puzzle2d@1/*#editor#select");
    assert_eq!(mutations, puzzle2d_selection_yields(&base, &records, 12.0).into_iter().map(|(_, mutation)| mutation).collect::<Vec<_>>(), "the transaction holds exactly the yields, in yield order");
    let (other, _) = commit("seed-2", &base, 12.0, records).expect("a second admission commits");
    assert_ne!(other.id, transaction.id, "two admissions never share a transaction id");
}

#[test]
fn several_records_in_one_flush_commit_as_one_transaction() {
    let records = vec![Puzzle2dSelectionRecord::drag(ids(&["left"]), 5.0, 0.0), Puzzle2dSelectionRecord::drag(ids(&["region-1"]), 0.0, 5.0)];
    let (_, mutations) = commit("seed-1", &board(), 0.0, records).expect("both records commit");
    assert_eq!(mutations.len(), 2, "one transaction carries both leaves: {mutations:?}");
}

#[test]
fn all_locked_identity_missing_and_empty_requests_leave_zero_trace() {
    let base = board();
    let locked = Puzzle2dSelectionRecord::drag(ids(&["locked"]), 5.0, 5.0);
    assert!(locked.refused_as_locked(&base), "every target locked is the tool-level refusal");
    assert_eq!(commit("seed", &base, 0.0, vec![locked]), None);
    let missing = Puzzle2dSelectionRecord::drag(ids(&["ghost"]), 5.0, 5.0);
    assert!(!missing.refused_as_locked(&base), "a missing target is no lock");
    assert_eq!(commit("seed", &base, 0.0, vec![missing]), None);
    for identity in [Puzzle2dSelectionMotion::Drag { dx: 0.0, dy: 0.0 }, Puzzle2dSelectionMotion::Rotate { pivot_x: 0.0, pivot_y: 0.0, angle: 0.0 }, Puzzle2dSelectionMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, factor: 1.0 }] {
        let record = Puzzle2dSelectionRecord { targets: ids(&["left"]), motion: identity, proximity: Vec::new(), connect: true };
        assert_eq!(commit("seed", &base, 0.0, vec![record]), None, "{identity:?} moves nothing");
    }
    let regions_only = Puzzle2dSelectionRecord { targets: ids(&["region-1"]), motion: Puzzle2dSelectionMotion::Rotate { pivot_x: 0.0, pivot_y: 0.0, angle: 1.0 }, proximity: Vec::new(), connect: false };
    assert_eq!(commit("seed", &base, 0.0, vec![regions_only]), None, "an axis-aligned region never rotates");
    assert_eq!(commit("seed", &base, 0.0, Vec::new()), None);
}

#[test]
fn a_mixed_request_yields_whole_and_its_leaf_reports_the_locked_rest_partial() {
    let base = board();
    let record = Puzzle2dSelectionRecord { connect: false, ..Puzzle2dSelectionRecord::drag(ids(&["left", "locked"]), 5.0, 0.0) };
    assert!(record.applies_to(&base) && !record.refused_as_locked(&base));
    let (_, mutations) = commit("seed", &base, 0.0, vec![record]).expect("a movable target commits the whole record");
    assert_eq!(mutations, vec![Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["left", "locked"]), dx: 5.0, dy: 0.0 })], "the locked target stays a literal input");
    let outcome = mutations[0].diff(&base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.partial"), "the leaf reports the locked target: {:?}", outcome.messages());
}

#[test]
fn the_pivot_is_the_targets_centroid_with_region_centres_on_request() {
    let base = board();
    assert_eq!(puzzle2d_selection_pivot(&base, &ids(&["left", "right"]), false), Some((500.0, 0.0)));
    assert_eq!(puzzle2d_selection_pivot(&base, &ids(&["left", "region-1"]), false), Some((0.0, 0.0)), "a rotation pivot ignores regions");
    assert_eq!(puzzle2d_selection_pivot(&base, &ids(&["left", "region-1"]), true), Some((10.0, 10.0)), "a scale pivot averages the region centre too");
    assert_eq!(puzzle2d_selection_pivot(&base, &ids(&["ghost"]), true), None);
}

#[test]
fn the_select_tool_rests_idle_and_streams_one_gesture() {
    let definition = <select_tool::SelectTool as machine::Machine>::definition();
    assert_eq!(definition.id, "select_tool");
    assert_eq!(definition.nodes.iter().skip(1).map(|node| node.stable_id).collect::<Vec<_>>(), vec!["idle", "streaming"]);
    assert_eq!(definition.transitions.len(), 5, "idle: records, stream; streaming: stream, finish, cancel");
    assert!(definition.transitions.iter().take(2).all(|transition| transition.guard.is_some() && transition.actions.len() == 1), "every gesture opens behind a guard and yields");
}

/// 🌊️ One dispatch of a streamed `translateSelection` against the gesture its window holds.
fn drive(held: Option<&GestureState<Puzzle2dMutation>>, phase: GesturePhase, request: Option<SelectToolRequest>, revision: &str) -> GestureDrive<GestureState<Puzzle2dMutation>, Puzzle2dMutation> {
    drive_chart_gesture::<select_tool::SelectTool>(held, "translateSelection", phase, request, "seed-1", revision).expect("the select tool drives")
}

#[test]
fn a_streamed_gesture_spans_dispatches_and_commits_one_transaction() {
    let base = board();
    let opened = drive(None, GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -400.0, 0.0)])), "rev-1");
    assert!(opened.committed.is_none() && !opened.continued, "the first tick opens a gesture and publishes nothing");
    let first = opened.next.flatten().expect("an open gesture is the window's next gesture");
    assert_eq!((first.states.as_slice(), first.verb.as_str(), first.base_revision.as_str(), &first.context), (&["root".to_string(), "streaming".to_string()][..], "translateSelection", "rev-1", &semio_framework_value::DslValue::Bool(true)));
    assert_eq!(first.entries, vec![(PUZZLE2D_SELECT_TOOL_LEAF_KEY.to_string(), Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: -400.0, dy: 0.0 }))], "a stream holds ONE net leaf");
    let ticked = drive(Some(&first), GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -548.0, 0.0)])), "rev-1");
    assert!(ticked.committed.is_none() && ticked.continued, "a second tick continues the held gesture");
    let second = ticked.next.flatten().expect("the gesture is still open");
    assert_eq!(second.transaction, first.transaction, "every tick of one gesture joins ONE transaction");
    assert_eq!(second.entries, vec![(PUZZLE2D_SELECT_TOOL_LEAF_KEY.to_string(), Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: -948.0, dy: 0.0 }))], "the ticks add up into the one leaf");
    let finished = drive(Some(&second), GesturePhase::Commit, Some(request(&base, Vec::new())), "rev-1");
    let (transaction, mutations) = finished.committed.expect("the commit publishes the gesture");
    assert_eq!(transaction, first.transaction, "the commit publishes the ref minted at the first tick");
    assert_eq!(transaction.tool, "s.puzzle.puzzle2d@1/*#editor#translateSelection");
    assert_eq!(mutations, puzzle2d_selection_yields(&base, &[Puzzle2dSelectionRecord::drag(ids(&["right"]), -948.0, 0.0)], 12.0).into_iter().map(|(_, mutation)| mutation).collect::<Vec<_>>(), "the net leaf plus the connection its drop lands");
    assert_eq!(finished.next, Some(None), "a committed gesture leaves the window's slot empty");
}

#[test]
fn a_host_abort_or_a_moved_base_mid_gesture_leaves_zero_trace() {
    let base = board();
    let open = drive(None, GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -400.0, 0.0)])), "rev-1").next.flatten().expect("open");
    for reason in [ToolAbortReason::Blur, ToolAbortReason::CaptureLost, ToolAbortReason::BaseMoved, ToolAbortReason::Frozen, ToolAbortReason::Retired] {
        let aborted = drive(Some(&open), GesturePhase::Abort(reason), None, "rev-1");
        assert!(aborted.committed.is_none() && aborted.next == Some(None), "{reason:?} drops the open gesture and publishes nothing");
    }
    let moved = drive(Some(&open), GesturePhase::Commit, Some(request(&base, Vec::new())), "rev-2");
    assert!(moved.committed.is_none() && moved.next == Some(None), "a commit on a moved base commits nothing and drops the gesture");
    let resting = drive(None, GesturePhase::Abort(ToolAbortReason::Blur), None, "rev-1");
    assert!(resting.committed.is_none() && resting.next.is_none(), "aborting a resting tool is a no-op");
}

#[test]
fn a_tampered_or_resting_gesture_never_resumes() {
    let base = board();
    let open = drive(None, GesturePhase::Stream, Some(request(&base, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), -400.0, 0.0)])), "rev-1").next.flatten().expect("open");
    let resting = GestureState { states: vec!["root".into(), "idle".into()], ..open.clone() };
    assert!(matches!(<ChartGesture<select_tool::SelectTool> as GestureTool>::resume(&resting), Err(ToolRefusal::Unclosed)), "a resting chart with an open transaction would let the next gesture join it");
    let unknown = GestureState { states: vec!["root".into(), "dragging".into()], ..open.clone() };
    assert!(<ChartGesture<select_tool::SelectTool> as GestureTool>::resume(&unknown).is_err(), "a configuration the chart does not know is refused");
    let dropped = drive(Some(&unknown), GesturePhase::Commit, Some(request(&base, Vec::new())), "rev-1");
    assert!(dropped.committed.is_none() && dropped.next == Some(None), "a gesture the tool cannot restore is dropped with zero trace");
}

/// 🎚️ LAW (F21): a committed leaf states its inputs at the precision their schema declares, whatever a host measured —
/// an `f32` drag of 60 recorded as 59.99996 is the row "by (60, 0)", and a motion that rounds to the identity is no
/// edit at all.
#[test]
fn a_committed_leaf_states_its_inputs_at_their_declared_precision() {
    let base = board();
    let (_, dragged) = commit("seed-1", &base, 0.0, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), 59.99996, -0.004)]).expect("the drag commits");
    assert_eq!(dragged, vec![Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: 60.0, dy: 0.0 })]);
    let scale = Puzzle2dSelectionRecord { targets: ids(&["right"]), motion: Puzzle2dSelectionMotion::Scale { pivot_x: 10.004999, pivot_y: -0.001, factor: 1.4999999 }, proximity: Vec::new(), connect: false };
    let (_, scaled) = commit("seed-1", &base, 0.0, vec![scale]).expect("the scaling commits");
    assert_eq!(Puzzle2dSelectionRecord::from_leaf(&scaled[0], false).map(|record| record.motion), Some(Puzzle2dSelectionMotion::Scale { pivot_x: 10.0, pivot_y: 0.0, factor: 1.5 }));
    assert_eq!(commit("seed-1", &base, 0.0, vec![Puzzle2dSelectionRecord::drag(ids(&["right"]), 0.004, -0.004)]), None, "a drag that rounds to nothing leaves zero trace");
    let exact = Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["right"]), dx: 12.25, dy: -7.5 });
    assert_eq!(puzzle2d_declared_precision(exact.clone()), exact, "an input already at its precision is untouched");
}

#[test]
fn repeated_targets_yield_one_leaf_over_unique_targets() {
    let base = board();
    let (_, mutations) = commit("seed", &base, 0.0, vec![Puzzle2dSelectionRecord { connect: false, ..Puzzle2dSelectionRecord::drag(ids(&["left", "left", "region-1", "left"]), 5.0, 0.0) }]).expect("a drag with repeats commits");
    assert_eq!(mutations, vec![Puzzle2dMutation::DragSelection(DragSelection { targets: ids(&["left", "region-1"]), dx: 5.0, dy: 0.0 })], "each target once, in first-seen order");
    assert!(mutations[0].diff(&base).is_applicable(protocol::MergePolicy::default()), "the leaf passes its own invariants: {:?}", mutations[0].diff(&base).messages());
    let rotate = Puzzle2dSelectionRecord { targets: ids(&["left", "right", "left"]), motion: Puzzle2dSelectionMotion::Rotate { pivot_x: 0.0, pivot_y: 0.0, angle: 1.0 }, proximity: Vec::new(), connect: false };
    assert_eq!(rotate.mutation(), rotate_selection(["left".into(), "right".into()].into_iter().collect(), 0.0, 0.0, 1.0), "a literal record's leaf is deduplicated too");
    let decoded = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "g", "kind": "drag", "targets": ["left", "left"], "dx": 1.0, "dy": 0.0, "proximity": [] })).expect("decodes");
    assert_eq!(decoded.targets, ids(&["left"]), "a board record decodes deduplicated");
}

#[test]
fn target_less_and_inadmissible_records_leave_zero_trace_and_no_lock_refusal() {
    let base = board();
    assert_eq!(Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "g", "kind": "drag", "targets": [], "dx": 1.0, "dy": 0.0, "proximity": [] })), None, "a target-less record is malformed");
    for factor in [0.0, -2.0] {
        assert_eq!(Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "g", "kind": "scale", "targets": ["left"], "pivotX": 0.0, "pivotY": 0.0, "factor": factor, "proximity": [] })), None, "factor {factor} is malformed");
    }
    assert_eq!(commit("seed", &base, 0.0, vec![Puzzle2dSelectionRecord::drag(Vec::new(), 5.0, 0.0)]), None, "no target, no transaction");
    for motion in [
        Puzzle2dSelectionMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, factor: 0.0 },
        Puzzle2dSelectionMotion::Scale { pivot_x: 0.0, pivot_y: 0.0, factor: -1.0 },
        Puzzle2dSelectionMotion::Drag { dx: f64::NAN, dy: 0.0 },
        Puzzle2dSelectionMotion::Rotate { pivot_x: 0.0, pivot_y: 0.0, angle: f64::INFINITY },
    ] {
        let record = Puzzle2dSelectionRecord { targets: ids(&["left", "locked"]), motion, proximity: Vec::new(), connect: false };
        assert!(!record.applies_to(&base) && !record.refused_as_locked(&base), "{motion:?} is inadmissible, not a lock refusal");
        assert_eq!(commit("seed", &base, 0.0, vec![record]), None, "{motion:?} leaves zero trace");
    }
}
