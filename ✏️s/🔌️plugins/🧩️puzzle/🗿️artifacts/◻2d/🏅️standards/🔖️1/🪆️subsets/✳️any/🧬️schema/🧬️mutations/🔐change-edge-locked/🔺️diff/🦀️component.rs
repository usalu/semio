//! 🔺️ Sparse diff builder for `ChangeEdgeLocked` — patches the one addressed edge in place.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgePatch, Puzzle2dEdgePatchEntry, Puzzle2dEdgesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeEdgeLocked, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle2dDiff::default();
    };
    let mut next = edge.clone();
    next.locked = payload.new_locked;
    Puzzle2dDiff {
        edges: Some(Puzzle2dEdgesDelta { patched: vec![Puzzle2dEdgePatchEntry { id: payload.id.clone(), patch: Puzzle2dEdgePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
