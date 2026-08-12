//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches `meta.kindCatalogs`.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    let mut meta = base.meta.clone();
    meta.kind_catalogs = payload.new_catalogs.clone();
    Puzzle2dDiff { meta: Some(meta), ..Default::default() }
}
//#endregion 🔖️Diff
