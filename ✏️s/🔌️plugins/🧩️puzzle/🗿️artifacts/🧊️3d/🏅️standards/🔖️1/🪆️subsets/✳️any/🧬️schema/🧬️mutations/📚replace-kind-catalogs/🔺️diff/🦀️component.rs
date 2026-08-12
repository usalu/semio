//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches the document `meta.kindCatalogs`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceKindCatalogs, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    let mut meta = base.meta.clone();
    meta.kind_catalogs = payload.new_catalogs.clone();
    Puzzle3dDiff { meta: Some(meta), ..Default::default() }
}
//#endregion 🔖️Diff
