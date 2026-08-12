//! 🔺️ Sparse diff builder for `DisconnectVortices` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle3d::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectVortices, _base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    Puzzle3dDiff { attractions: Some(Puzzle3dAttractionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
