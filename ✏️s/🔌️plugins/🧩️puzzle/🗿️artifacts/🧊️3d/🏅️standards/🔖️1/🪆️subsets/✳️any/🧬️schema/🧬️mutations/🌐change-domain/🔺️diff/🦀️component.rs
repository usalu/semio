//! 🔺️ Sparse diff builder for `ChangeDomain` — patches the document `domain`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDomain, _base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    Puzzle3dDiff { domain: Some(payload.new_domain.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
