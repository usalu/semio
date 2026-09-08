use super::*;
use crate::{Puzzle2dEdge, Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};
use protocol::Inference;

//#region 🧸️Fixtures
fn parent_child_snapshot() -> Puzzle2dSnapshot {
    // p (Fixed, off-origin) --e-- c (Derived): edge x/y offsets place c relative to p.
    let p = Puzzle2dNode { id: "p".into(), x: 5.0, y: 7.0, anchor: Puzzle2dNodeAnchor::Fixed, handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }], ..Default::default() };
    let c = Puzzle2dNode { id: "c".into(), anchor: Puzzle2dNodeAnchor::Derived, handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }], ..Default::default() };
    let e = Puzzle2dEdge { id: "e".into(), source: "p:h".into(), target: "c:h".into(), x: 3.0, y: -2.0, ..Default::default() };
    Puzzle2dSnapshot { schema: crate::PUZZLE_2D_SCHEMA.to_string(), camera: Default::default(), nodes: vec![p, c], edges: vec![e], meta: Default::default() }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = parent_child_snapshot();
    assert_eq!(Puzzle2dInference::infer(&snapshot), Puzzle2dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(Puzzle2dInference::infer(&Puzzle2dSnapshot::default()), Puzzle2dInference::default());
}

#[test]
fn inference_matches_compute_flat_position_directly() {
    let snapshot = parent_child_snapshot();
    let inferred = Puzzle2dInference::infer(&snapshot);
    assert_eq!(inferred.flat_position, compute_flat_position(&snapshot));
}
//#endregion 🧪️InferenceLaws
