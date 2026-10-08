//! 🛠️ Laws of the transform tool machine alone: a gesture record yields its parametric leaf (and a drop's
//! attractions) as ONE committed `ToolTransaction` under a deterministic ref, a gesture that moves nothing leaves
//! zero trace, and every host payload states the RELATIVE motion from its own start pose.

use super::*;
use crate::apply_puzzle3d_mutation;
use semio_framework_tool_machine::{ToolMachineRunner, ToolStep};
use crate::standards::v1::subsets::any::schema::mutations::{quat_from_axis_angle,quat_mul};

use crate::{Puzzle3dObject, Puzzle3dTargetVolume, Puzzle3dVortex};

/// 🧱️ Two objects facing each other along x — `a` locked-free with vortex `a:v` at +x, `b` with vortex `b:v` at -x —
/// a locked `pin`, and one target volume.
fn scene() -> Puzzle3dSnapshot {
    let vortex = |id: &str, x: f64| Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position: [x, 0.0, 0.0], direction: Some([x.signum(), 0.0, 0.0]), radius: None, hidden: false, locked: false };
    let object = |id: &str, x: f64, vortices: Vec<Puzzle3dVortex>, locked: bool| Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor: Default::default(), origin: [x, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices, hidden: false, locked };
    Puzzle3dSnapshot {
        schema: crate::PUZZLE_3D_SCHEMA.into(),
        objects: vec![object("a", 0.0, vec![vortex("v", 1.0)], false), object("b", 10.0, vec![vortex("v", -1.0)], false), object("pin", 20.0, Vec::new(), true)],
        target_volumes: vec![Puzzle3dTargetVolume { id: "box".into(), origin: [0.0; 3], orientation: None, scale: None, hidden: false, locked: false }],
        ..Default::default()
    }
}

fn clock(physical_ms: u64) -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms, logical: 0 }
}

fn commit(verb: &str, base: &Puzzle3dSnapshot, records: Vec<Puzzle3dSelectionRecord>) -> Option<(protocol::TransactionRef, Vec<Puzzle3dMutation>)> {
    puzzle3d_transform_tool_commit(verb, "seed", TransformToolRequest { base: Arc::new(base.clone()), records })
}

#[test]
fn a_gumball_delta_is_one_transaction_of_its_parametric_leaf() {
    let base = scene();
    let record = Puzzle3dSelectionRecord::from_gumball("translateSelection", Some(&json!({ "dx": 2.0, "dy": 0.0, "dz": -1.0 })), ["a".to_string(), "box".to_string(), "a".to_string()]).expect("a finite delta decodes");
    let (transaction, mutations) = commit("translateSelection", &base, vec![record.clone()]).expect("a moving record commits");
    assert_eq!(mutations, vec![crate::standards::v1::subsets::any::schema::mutations::drag_selection(vec!["a".into(), "box".into()], [2.0, 0.0, -1.0])], "one leaf over the deduplicated targets");
    assert_eq!(transaction.tool, "s.puzzle.puzzle3d@1/*#editor#translateSelection");
    assert_ne!(commit("translateSelection", &base, vec![record]).map(|(again, _)| again), Some(transaction), "each released gesture mints its own transaction even within one clock millisecond");
}

#[test]
fn a_gesture_that_moves_nothing_leaves_zero_trace() {
    let base = scene();
    let still = Puzzle3dSelectionRecord::new(["a".to_string()], Puzzle3dSelectionMotion::Drag { offset: [0.0; 3] });
    let ghosts = Puzzle3dSelectionRecord::new(["ghost".to_string()], Puzzle3dSelectionMotion::Drag { offset: [1.0, 0.0, 0.0] });
    let pinned = Puzzle3dSelectionRecord::new(["pin".to_string()], Puzzle3dSelectionMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 1.0 });
    let flat = Puzzle3dSelectionRecord::new(["a".to_string()], Puzzle3dSelectionMotion::Scale { factors: [1.0, 0.0, 1.0] });
    for record in [still, ghosts, pinned.clone(), flat] {
        assert!(commit("translateSelection", &base, vec![record.clone()]).is_none(), "{record:?} must leave zero trace");
    }
    assert!(pinned.refused_as_locked(&base), "an all-locked gesture is the lock refusal, not a silent drop");
}

#[test]
fn a_relocate_drop_yields_its_drag_and_the_attraction_it_lands() {
    let base = scene();
    let record = puzzle3d_relocate_record(&base, "b", [2.5, 0.0, 0.0], 0.75).expect("b is in the scene");
    assert_eq!(record.motion, Puzzle3dSelectionMotion::Drag { offset: [-7.5, 0.0, 0.0] }, "the drop is the offset from b's base origin");
    assert_eq!(record.attractions, vec![("a:v".to_string(), "b:v".to_string())], "b's vortex lands on a's: the stationary one attracts");
    let (_, mutations) = commit("worldRelocate", &base, vec![record]).expect("the drop commits");
    assert_eq!(mutations.len(), 2, "the drag and one attraction: {mutations:?}");
    let Puzzle3dMutation::ConnectVortices(connect) = &mutations[1] else { panic!("the attraction follows the drag: {mutations:?}") };
    assert_eq!((connect.id.as_str(), connect.attracting.as_str(), connect.attracted.as_str()), ("attraction-a:v-b:v", "a:v", "b:v"), "the id is minted from the pair, never a counter");
    let mut moved = base.clone();
    for mutation in &mutations {
        apply_puzzle3d_mutation(&mut moved, mutation).expect("the transaction applies");
    }
    assert_eq!(moved.objects[1].origin, [2.5, 0.0, 0.0], "the dropped object lands where it was dropped");
    assert!((connect.gap - 0.5).abs() < 1e-9 && connect.shift.abs() < 1e-9 && connect.rise.abs() < 1e-9, "the parameters reproduce the dropped pose, so resolving never jumps it: {connect:?}");
    assert!(puzzle3d_relocate_record(&moved, "b", [2.5, 0.0, 0.0], 0.75).expect("b").attractions.is_empty(), "an attracted pair is never attracted twice");
}

#[test]
fn a_volume_relocate_states_the_relative_motion_from_its_start_pose() {
    let pose = |position: [f64; 3], quaternion: [f64; 4], scale: [f64; 3]| json!({ "position": position, "quaternion": quaternion, "scale": scale });
    let identity = [0.0, 0.0, 0.0, 1.0];
    let moved = Puzzle3dSelectionRecord::from_pose_delta(Some(&json!({ "volumeId": "box", "mode": "translate", "before": pose([1.0, 2.0, 3.0], identity, [1.0; 3]), "after": pose([4.0, 2.0, 0.0], identity, [1.0; 3]) }))).expect("translate");
    assert_eq!((moved.targets, moved.motion), (vec!["box".to_string()], Puzzle3dSelectionMotion::Drag { offset: [3.0, 0.0, -3.0] }));
    let start = quat_from_axis_angle(1.0, 0.0, 0.0, 0.4);
    let end = quat_mul(quat_from_axis_angle(0.0, 0.0, 1.0, 1.2), start);
    let turned = Puzzle3dSelectionRecord::from_pose_delta(Some(&json!({ "volumeId": "box", "mode": "rotate", "before": pose([0.0; 3], start, [1.0; 3]), "after": pose([0.0; 3], end, [1.0; 3]) }))).expect("rotate");
    let Puzzle3dSelectionMotion::Rotate { axis, angle } = turned.motion else { panic!("{turned:?}") };
    let reached = quat_mul(quat_from_axis_angle(axis[0], axis[1], axis[2], angle), start);
    assert!(reached.iter().zip(end).all(|(reached, end)| (reached - end).abs() < 1e-12), "the recorded turn carries the start orientation onto the end one: {reached:?} vs {end:?}");
    let grown = Puzzle3dSelectionRecord::from_pose_delta(Some(&json!({ "volumeId": "box", "mode": "scale", "before": pose([0.0; 3], identity, [2.0, 2.0, 2.0]), "after": pose([0.0; 3], identity, [4.0, 1.0, 2.0]) }))).expect("scale");
    assert_eq!(grown.motion, Puzzle3dSelectionMotion::Scale { factors: [2.0, 0.5, 1.0] });
    assert!(Puzzle3dSelectionRecord::from_pose_delta(Some(&json!({ "volumeId": "box", "mode": "scale", "before": pose([0.0; 3], identity, [0.0, 1.0, 1.0]), "after": pose([0.0; 3], identity, [1.0; 3]) }))).is_none(), "a degenerate start scale states no admissible factor");
}

#[test]
fn the_transform_tool_is_a_one_state_statechart_that_commits_per_event() {
    let definition = <transform_tool::TransformTool as machine::Machine>::definition();
    assert_eq!(definition.fingerprint, <transform_tool::TransformTool as machine::Machine>::definition().fingerprint, "the chart's identity is stable");
    let base = scene();
    let record = Puzzle3dSelectionRecord::new(["a".to_string()], Puzzle3dSelectionMotion::Scale { factors: [2.0, 2.0, 2.0] });
    let mut runner = ToolMachineRunner::<transform_tool::TransformTool, TransformToolHost>::start(format!("{PUZZLE3D_EDITOR_APP_ID}#scaleSelection"), protocol::ActorId("seed".into()), TransformToolContext, TransformToolHost).expect("the tool starts at rest");
    for tick in 0..2u64 {
        let step = runner.send(transform_tool::Event::Records(TransformToolRequest { base: Arc::new(base.clone()), records: vec![record.clone()] }), clock(tick)).expect("an event is admitted");
        assert!(matches!(step, ToolStep::Committed(..)), "every release is its own committed transaction: {step:?}");
        assert!(runner.at_rest() && runner.transaction().is_none_or(|transaction| transaction.state() != semio_framework_tool_machine::ToolTransactionState::Open), "nothing stays open between gestures");
    }
}

/// 📄️ Paging changes WHEN a drop's proximity scan runs, never WHAT it finds: for every page size the paged scan
/// reports monotone progress, needs exactly `ceil(objects / page)` steps and ends on the very record the one-call
/// scan finds — the same attracted pairs in the same order — and an already attracted pair stays excluded.
#[test]
fn a_paged_relocate_scan_finds_exactly_what_the_one_call_scan_finds() {
    let mut base = scene();
    let vortex = |x: f64| Puzzle3dVortex { id: "v".into(), vortex_kind: None, label: None, position: [x, 0.0, 0.0], direction: Some([1.0, 0.0, 0.0]), radius: None, hidden: false, locked: false };
    for (index, y) in [0.2, -0.3, 0.6, 2.0, 0.1].into_iter().enumerate() {
        base.objects.push(Puzzle3dObject { id: format!("n{index}"), label: None, object_kind: None, anchor: Default::default(), origin: [1.5, y, 0.0], orientation: None, scale: None, mesh_url: None, vortices: vec![vortex(0.0)], hidden: false, locked: false });
    }
    base.attractions.push(crate::Puzzle3dAttraction { id: "held".into(), attracting: "n4:v".into(), attracted: "b:v".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    let whole = puzzle3d_relocate_record(&base, "b", [2.5, 0.0, 0.0], 0.75).expect("b is in the scene");
    assert_eq!(whole.attractions, vec![("a:v".to_string(), "b:v".to_string()), ("n0:v".to_string(), "b:v".to_string()), ("n1:v".to_string(), "b:v".to_string()), ("n2:v".to_string(), "b:v".to_string())], "every free vortex within the radius, in document order; `n4` is already attracted and `n3` is out of reach");
    for page in 1..=base.objects.len() + 1 {
        let mut scan = Puzzle3dRelocateScan::begin(&base, "b", [2.5, 0.0, 0.0], 0.75).expect("b is in the scene");
        let (mut steps, mut measured) = (0, 0);
        while !scan.step(&base, page) {
            steps += 1;
            let (done, total) = scan.progress(&base);
            assert!(done > measured && done < total, "page {page}: progress is monotone and never claims the end early");
            measured = done;
        }
        assert_eq!(steps + 1, base.objects.len().div_ceil(page), "page {page}: one step per page");
        assert_eq!(scan.progress(&base), (base.objects.len(), base.objects.len()));
        assert_eq!(scan.finish(), whole, "page {page}: the paged scan ends on the one-call record");
    }
    let lonely = Puzzle3dRelocateScan::begin(&base, "pin", [0.0; 3], 0.75).expect("pin is in the scene");
    assert_eq!(lonely.progress(&base), (base.objects.len(), base.objects.len()), "an object without a vortex attracts nothing, so its scan starts done");
}

#[test]
fn the_transform_chart_obeys_the_shared_phase_fixture() {
    let fixture = semio_framework_pack_json::parse(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🛠️transform-gesture/🔣️.json")), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture");
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let values = case.get("offset").unwrap().as_array().unwrap();
        let offset = [values[0].as_f64().unwrap(), values[1].as_f64().unwrap(), values[2].as_f64().unwrap()];
        let phase = match case.get("phase").unwrap().as_str().unwrap() {
            "once" => GesturePhase::Once,
            "stream" => GesturePhase::Stream,
            "commit" => GesturePhase::Commit,
            "abort" => GesturePhase::Abort(semio_framework_tool_machine::ToolAbortReason::Frozen),
            _ => unreachable!(),
        };
        let request = TransformToolRequest { base: Arc::new(scene()), records: vec![Puzzle3dSelectionRecord::new(["a".to_string()], Puzzle3dSelectionMotion::Drag { offset })] };
        let drive = semio_framework_tool_machine::drive_chart_gesture::<transform_tool::TransformTool>(None, "translateSelection", phase, Some(request), "seed", "base").expect("one dispatch");
        assert_eq!(drive.committed.as_ref().map_or(0, |(_, leaves)| leaves.len()), case.get("committed").unwrap().as_u64().unwrap() as usize);
        assert!(drive.next.is_none(), "the release-only chart holds no transaction");
    }
}
