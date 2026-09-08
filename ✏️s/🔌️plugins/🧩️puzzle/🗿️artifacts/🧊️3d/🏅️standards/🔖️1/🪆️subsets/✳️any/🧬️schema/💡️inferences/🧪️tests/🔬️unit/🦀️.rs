
use super::*;
use crate::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dVortex};
use protocol::Inference;

//#region 🧸️Fixtures
fn vortex(id: &str, position: [f64; 3], direction: [f64; 3]) -> Puzzle3dVortex {
    Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position, direction: Some(direction), radius: None, hidden: false, locked: false }
}

fn object(id: &str, origin: [f64; 3], anchor: Puzzle3dObjectAnchor, vortices: Vec<Puzzle3dVortex>) -> Puzzle3dObject {
    Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor, origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, vortices, hidden: false, locked: false }
}

fn chain_snapshot() -> Puzzle3dSnapshot {
    // root -A- mid -B- leaf: a 3-object chain — same shape as the flat-position slug's own
    // fixture, kept independent per-file per this repo's inline-fixture convention.
    let root = object("root", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Fixed, vec![vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
    let mid = object("mid", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]), vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
    let leaf = object("leaf", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0])]);
    let attraction_a = Puzzle3dAttraction { id: "a1".into(), attracting: "root:top".into(), attracted: "mid:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.0, y: 0.0 };
    let attraction_b = Puzzle3dAttraction { id: "a2".into(), attracting: "mid:top".into(), attracted: "leaf:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 1.0 };
    Puzzle3dSnapshot {
        schema: crate::PUZZLE_3D_SCHEMA.to_string(),
        domain: "architecture".into(),
        meta: Default::default(),
        objects: vec![root, mid, leaf],
        attractions: vec![attraction_a, attraction_b],
        target_volumes: Vec::new(),
        references: Vec::new(),
    }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(Puzzle3dInference::infer(&snapshot), Puzzle3dInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(Puzzle3dInference::infer(&Puzzle3dSnapshot::default()), Puzzle3dInference::default());
}

#[test]
fn inference_matches_flatten_snapshot_directly() {
    let snapshot = chain_snapshot();
    let inferred = Puzzle3dInference::infer(&snapshot);
    let direct = flatten_snapshot(&snapshot);
    for (id, pose) in &direct {
        assert_eq!(inferred.flat_positions.get(id), Some(pose), "inference must match flatten_snapshot exactly for {id}");
    }
}
//#endregion 🧪️InferenceLaws
