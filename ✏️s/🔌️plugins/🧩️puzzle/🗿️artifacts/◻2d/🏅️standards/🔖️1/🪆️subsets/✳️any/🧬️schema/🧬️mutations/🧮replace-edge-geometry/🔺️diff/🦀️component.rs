//! 🔺️ Sparse diff builder for `ReplaceEdgeGeometry` — patches the one addressed edge's connection
//! pose.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgePatch, Puzzle2dEdgePatchEntry, Puzzle2dEdgesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceEdgeGeometry, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle2dDiff::default();
    };
    let mut next = edge.clone();
    next.gap = payload.new_gap;
    next.shift = payload.new_shift;
    next.rise = payload.new_rise;
    next.rotation = payload.new_rotation;
    next.turn = payload.new_turn;
    next.tilt = payload.new_tilt;
    next.x = payload.new_x;
    next.y = payload.new_y;
    Puzzle2dDiff {
        edges: Some(Puzzle2dEdgesDelta { patched: vec![Puzzle2dEdgePatchEntry { id: payload.id.clone(), patch: Puzzle2dEdgePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
