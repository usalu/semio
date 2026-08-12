//! 🔺️ Sparse diff builder for `DisconnectHandles` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectHandles, _base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
