//! 🔺️ Sparse diff builder for `ChangeDomain` — patches the document `domain`.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDomain, _base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    Puzzle5dDiff { domain: Some(payload.new_domain.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
