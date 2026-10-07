//! 🛠️ Laws of the puzzle 5d transform tool machine alone: a gesture record yields its parametric leaf (and a
//! drop's fastenings) as ONE committed `ToolTransaction` under a deterministic ref, a board drag stays on the
//! board, a gesture that moves nothing leaves zero trace.

use super::*;
use semio_framework_tool_machine::{ToolMachineRunner, ToolStep};
use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;
use crate::{Puzzle5dGrip, Puzzle5dGrip3d, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dTargetVolume};

/// 🧱️ Two parts facing each other along x — `a` with grip `a:g` at +x, `b` with grip `b:g` at -x — a part `pin`
/// whose board pin is locked, and one target volume.
fn scene() -> Puzzle5dSnapshot {
    let grip = |x: f64| Puzzle5dGrip { id: "g".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Puzzle5dGrip3d { position: [x, 0.0, 0.0], ..Default::default() } };
    let part = |id: &str, x: f64, grips: Vec<Puzzle5dGrip>, locked: bool| Puzzle5dPart {
        id: id.into(),
        part_2d: Puzzle5dPart2d { x: x * 48.0, locked: locked.then_some(true), ..Default::default() },
        part_3d: Puzzle5dPart3d { origin: [x, 0.0, 0.0], ..Default::default() },
        grips,
        ..Default::default()
    };
    Puzzle5dSnapshot {
        parts: vec![part("a", 0.0, vec![grip(1.0)], false), part("b", 10.0, vec![grip(-1.0)], false), part("pin", 20.0, Vec::new(), true)],
        target_volumes: vec![Puzzle5dTargetVolume { id: "box".into(), ..Default::default() }],
        ..Default::default()
    }
}

fn clock(physical_ms: u64) -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms, logical: 0 }
}

fn commit(verb: &str, base: &Puzzle5dSnapshot, records: Vec<Puzzle5dSelectionRecord>) -> Option<(protocol::TransactionRef, Vec<Puzzle5dMutation>)> {
    puzzle5d_transform_tool_commit(verb, "seed", TransformToolRequest { base: Arc::new(base.clone()), records })
}

#[test]
fn a_gumball_delta_is_one_transaction_of_its_parametric_leaf() {
    let base = scene();
    let world = Puzzle3dSelectionRecord::from_gumball("translateSelection", Some(&json!({ "dx": 2.0, "dy": 0.0, "dz": -1.0 })), ["a".to_string(), "box".to_string(), "a".to_string()]).expect("a finite delta decodes");
    let record = Puzzle5dSelectionRecord::world(world);
    let (transaction, mutations) = commit("translateSelection", &base, vec![record.clone()]).expect("a moving record commits");
    assert_eq!(mutations, vec![drag_selection_3d(vec!["a".into(), "box".into()], [2.0, 0.0, -1.0])], "one leaf over the deduplicated targets");
    assert_eq!(transaction.tool, "s.puzzle.puzzle5d@1/*#editor#translateSelection");
    assert_ne!(commit("translateSelection", &base, vec![record]).map(|(again, _)| again), Some(transaction), "each released gesture mints its own transaction even within one clock millisecond");
}

#[test]
fn a_board_drag_is_one_board_leaf_that_never_touches_the_world() {
    let base = scene();
    let records = vec![
        Puzzle5dSelectionRecord::new(["a".to_string(), "box".to_string()], Puzzle5dSelectionMotion::Board { dx: 12.0, dy: -6.0 }),
        Puzzle5dSelectionRecord::new(["b".to_string()], Puzzle5dSelectionMotion::Board { dx: 1.0, dy: 0.0 }),
    ];
    let (_, mutations) = commit("applyBoardEvents", &base, records).expect("the drags commit");
    assert_eq!(mutations, vec![drag_selection_2d(vec!["a".into(), "box".into()], 12.0, -6.0), drag_selection_2d(vec!["b".into()], 1.0, 0.0)], "every drag of the batch is a leaf of the ONE transaction");
    let mut moved = base.clone();
    for mutation in &mutations {
        apply_puzzle5d_mutation(&mut moved, mutation).expect("the transaction applies");
    }
    assert_eq!((moved.parts[0].part_2d.x, moved.parts[0].part_2d.y, moved.parts[0].part_3d.origin), (12.0, -6.0, [0.0; 3]), "a board drag is a plan edit");
    assert!(!Puzzle5dSelectionRecord::new(["box".to_string()], Puzzle5dSelectionMotion::Board { dx: 1.0, dy: 0.0 }).applies_to(&base), "the board paints no target volume");
}

#[test]
fn a_gesture_that_moves_nothing_leaves_zero_trace() {
    let base = scene();
    let still = Puzzle5dSelectionRecord::new(["a".to_string()], Puzzle5dSelectionMotion::Board { dx: 0.0, dy: 0.0 });
    let ghosts = Puzzle5dSelectionRecord::new(["ghost".to_string()], Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Drag { offset: [1.0, 0.0, 0.0] }));
    let pinned = Puzzle5dSelectionRecord::new(["pin".to_string()], Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Rotate { axis: [0.0, 0.0, 1.0], angle: 1.0 }));
    let flat = Puzzle5dSelectionRecord::new(["a".to_string()], Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Scale { factors: [1.0, 0.0, 1.0] }));
    let unbounded = Puzzle5dSelectionRecord::new(["a".to_string()], Puzzle5dSelectionMotion::Board { dx: f64::NAN, dy: 1.0 });
    for record in [still, ghosts.clone(), pinned.clone(), flat, unbounded] {
        assert!(commit("translateSelection", &base, vec![record.clone()]).is_none(), "{record:?} must leave zero trace");
    }
    assert!(pinned.refused_as_locked(&base), "an all-locked gesture is the lock refusal, not a silent drop");
    assert!(!ghosts.names_any(&base), "a gesture over absent ids names nothing");
}

#[test]
fn a_world_drop_yields_its_drag_and_the_fastening_it_lands() {
    let base = scene();
    let record = puzzle5d_relocate_record(&base, "b", [2.5, 0.0, 0.0], 0.75).expect("b is in the scene");
    assert_eq!(record.motion, Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Drag { offset: [-7.5, 0.0, 0.0] }), "the drop is the offset from b's base origin");
    assert_eq!(record.fastenings, vec![("b:g".to_string(), "a:g".to_string())], "b's grip lands on a's: the moved grip is the source");
    let (_, mutations) = commit("worldRelocate", &base, vec![record]).expect("the drop commits");
    assert_eq!(mutations, vec![drag_selection_3d(vec!["b".into()], [-7.5, 0.0, 0.0]), connect_grips("fastener-b:g-a:g".into(), "b:g".into(), "a:g".into(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)], "the id is minted from the pair, never a counter");
    let mut moved = base.clone();
    for mutation in &mutations {
        apply_puzzle5d_mutation(&mut moved, mutation).expect("the transaction applies");
    }
    assert_eq!(moved.parts[1].part_3d.origin, [2.5, 0.0, 0.0], "the dropped part lands where it was dropped");
    assert_eq!(moved.parts[1].part_2d.x, 480.0 - 7.5 / PUZZLE5D_FLAT_TO_WORLD, "its board pin follows the drop");
    assert!(puzzle5d_relocate_record(&moved, "b", [2.5, 0.0, 0.0], 0.75).expect("b").fastenings.is_empty(), "a fastened pair is never fastened twice");
    assert_eq!(puzzle5d_minted_fastener_id(&moved, "b:g", "a:g"), "fastener-b:g-a:g-2", "a taken id is suffixed past the document's ids");
    assert!(puzzle5d_relocate_record(&base, "b", [2.5, 0.0, 0.0], 0.1).expect("b").fastenings.is_empty(), "the proximity radius bounds the scan");
}

#[test]
fn the_transform_tool_is_a_one_state_statechart_that_commits_per_event() {
    let definition = <transform_tool::TransformTool as machine::Machine>::definition();
    assert_eq!(definition.fingerprint, <transform_tool::TransformTool as machine::Machine>::definition().fingerprint, "the chart's identity is stable");
    let base = scene();
    let record = Puzzle5dSelectionRecord::new(["a".to_string()], Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Scale { factors: [2.0, 2.0, 2.0] }));
    let mut runner = ToolMachineRunner::<transform_tool::TransformTool, TransformToolHost>::start(format!("{PUZZLE5D_EDITOR_APP_ID}#scaleSelection"), protocol::ActorId("seed".into()), TransformToolContext, TransformToolHost).expect("the tool starts at rest");
    for tick in 0..2u64 {
        let step = runner.send(transform_tool::Event::Records(TransformToolRequest { base: Arc::new(base.clone()), records: vec![record.clone()] }), clock(tick)).expect("an event is admitted");
        assert!(matches!(step, ToolStep::Committed(..)), "every release is its own committed transaction: {step:?}");
        assert!(runner.at_rest() && runner.transaction().is_none_or(|transaction| transaction.state() != semio_framework_tool_machine::ToolTransactionState::Open), "nothing stays open between gestures");
    }
}

/// 📄️ Paging changes WHEN a world drop's proximity scan runs, never WHAT it finds: for every page size the paged scan
/// reports monotone progress, needs exactly `ceil(parts / page)` steps and ends on the very record the one-call scan
/// finds — the same fastenings in the same order — and an already fastened pair stays excluded.
#[test]
fn a_paged_world_drop_scan_finds_exactly_what_the_one_call_scan_finds() {
    let mut base = scene();
    let grip = Puzzle5dGrip { id: "g".into(), grip_kind: None, grip_2d: Default::default(), grip_3d: Puzzle5dGrip3d { position: [0.0; 3], ..Default::default() } };
    for (index, y) in [0.2, -0.3, 0.6, 2.0, 0.1].into_iter().enumerate() {
        base.parts.push(Puzzle5dPart { id: format!("n{index}"), part_3d: Puzzle5dPart3d { origin: [1.5, y, 0.0], ..Default::default() }, grips: vec![grip.clone()], ..Default::default() });
    }
    base.fasteners.push(crate::Puzzle5dFastener { id: "held".into(), source: "n4:g".into(), target: "b:g".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    let whole = puzzle5d_relocate_record(&base, "b", [2.5, 0.0, 0.0], 0.75).expect("b is in the scene");
    assert_eq!(whole.fastenings, ["a:g", "n0:g", "n1:g", "n2:g"].map(|target| ("b:g".to_string(), target.to_string())), "every free grip within the radius, in document order; `n4` is already fastened and `n3` is out of reach");
    for page in 1..=base.parts.len() + 1 {
        let mut scan = Puzzle5dRelocateScan::begin(&base, "b", [2.5, 0.0, 0.0], 0.75).expect("b is in the scene");
        let (mut steps, mut measured) = (0, 0);
        while !scan.step(&base, page) {
            steps += 1;
            let (done, total) = scan.progress(&base);
            assert!(done > measured && done < total, "page {page}: progress is monotone and never claims the end early");
            measured = done;
        }
        assert_eq!(steps + 1, base.parts.len().div_ceil(page), "page {page}: one step per page");
        assert_eq!(scan.finish(), whole, "page {page}: the paged scan ends on the one-call record");
    }
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
        let request = TransformToolRequest { base: Arc::new(scene()), records: vec![Puzzle5dSelectionRecord::new(["a".to_string()], Puzzle5dSelectionMotion::Drag { offset })] };
        let drive = semio_framework_tool_machine::drive_chart_gesture::<transform_tool::TransformTool>(None, "translateSelection", phase, Some(request), "seed", "base").expect("one dispatch");
        assert_eq!(drive.committed.as_ref().map_or(0, |(_, leaves)| leaves.len()), case.get("committed").unwrap().as_u64().unwrap() as usize);
        assert!(drive.next.is_none(), "the release-only chart holds no transaction");
    }
}
