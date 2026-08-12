//! 🔺️ Sparse diff builder for `DeleteReference` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle3d::diff::{Puzzle3dReferencesDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteReference, _base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    Puzzle3dDiff { references: Some(Puzzle3dReferencesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
