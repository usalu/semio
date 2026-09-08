
use super::*;
use crate::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};
use protocol::Inference;

//#region 🧸️Fixtures
fn chain_snapshot() -> Puzzle5dSnapshot {
    // p -f- c: a 2-part chain — same shape as puzzle3d's own inference fixture, kept
    // independent per-file per this repo's inline-fixture convention.
    let parent = Puzzle5dPart {
        id: "p".into(),
        part_kind: None,
        anchor: Puzzle5dPartAnchor::Fixed,
        part_2d: Puzzle5dPart2d { x: 10.0, y: 20.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
        grips: vec![Puzzle5dGrip {
            id: "top".into(),
            grip_kind: None,
            grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
            grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 1.0], direction: Some([0.0, 0.0, 1.0]), radius: None, label: None },
        }],
    };
    let child = Puzzle5dPart {
        id: "c".into(),
        part_kind: None,
        anchor: Puzzle5dPartAnchor::Derived,
        part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
        grips: vec![Puzzle5dGrip {
            id: "bottom".into(),
            grip_kind: None,
            grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
            grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, -1.0], direction: Some([0.0, 0.0, -1.0]), radius: None, label: None },
        }],
    };
    let fastener = Puzzle5dFastener { id: "f".into(), source: "p:top".into(), target: "c:bottom".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.5, y: 2.5 };
    Puzzle5dSnapshot {
        schema: "puzzle.5d".into(),
        domain: "architecture".into(),
        label: None,
        meta: Puzzle5dMeta::default(),
        kind_catalogs: None,
        kind_catalogs_extra: None,
        kind_compatibility: Vec::new(),
        parts: vec![parent, child],
        fasteners: vec![fastener],
    }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(Puzzle5dInference::infer(&snapshot), Puzzle5dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(Puzzle5dInference::infer(&Puzzle5dSnapshot::default()), Puzzle5dInference::default());
}

#[test]
fn inference_matches_flatten_snapshot_directly() {
    let snapshot = chain_snapshot();
    let inferred = Puzzle5dInference::infer(&snapshot);
    let direct = flatten_snapshot(&snapshot);
    for (id, pose) in &direct {
        assert_eq!(inferred.flat_positions.get(id), Some(pose), "inference must match flatten_snapshot exactly for {id}");
    }
}
//#endregion 🧪️InferenceLaws
