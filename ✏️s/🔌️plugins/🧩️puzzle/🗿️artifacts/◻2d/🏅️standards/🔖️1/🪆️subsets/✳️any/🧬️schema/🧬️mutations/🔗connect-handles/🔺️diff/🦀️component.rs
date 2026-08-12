//! 🔺️ Sparse diff builder for `ConnectHandles` — a real append-only insert (never a
//! whole-snapshot capture). No-op when the id already exists in `base`.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectHandles, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    if base.edges.iter().any(|entry| entry.id == payload.id) {
        return Puzzle2dDiff::default();
    }
    let edge = Puzzle2dEdge {
        id: payload.id.clone(),
        source: payload.source.clone(),
        target: payload.target.clone(),
        edge_kind: payload.edge_kind.clone(),
        gap: payload.gap,
        shift: payload.shift,
        rise: payload.rise,
        rotation: payload.rotation,
        turn: payload.turn,
        tilt: payload.tilt,
        x: payload.x,
        y: payload.y,
        source_tip: payload.source_tip.clone(),
        target_tip: payload.target_tip.clone(),
        visible: None,
        locked: None,
    };
    Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta { added: vec![edge], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
