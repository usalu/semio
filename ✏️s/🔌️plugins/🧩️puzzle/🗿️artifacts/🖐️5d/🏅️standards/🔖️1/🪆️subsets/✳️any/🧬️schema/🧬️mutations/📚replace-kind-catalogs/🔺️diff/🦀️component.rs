//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches the document `kindCatalogs`.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceKindCatalogs, _base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    Puzzle5dDiff { kind_catalogs: Some(payload.new_catalogs.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
