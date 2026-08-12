//! 🔺️ Sparse diff builder for `DisconnectKindCompatibility` — patches the document `meta.kindCompatibility`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectKindCompatibility, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    if !base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return Puzzle3dDiff::default();
    }
    let mut meta = base.meta.clone();
    meta.kind_compatibility.retain(|row| !(row.source == payload.source && row.target == payload.target));
    Puzzle3dDiff { meta: Some(meta), ..Default::default() }
}
//#endregion 🔖️Diff
