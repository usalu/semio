//! 🔺️ Sparse diff builder for `DisconnectGrips` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectGrips, _base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    Puzzle5dDiff { fasteners: Some(Puzzle5dFastenersDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
