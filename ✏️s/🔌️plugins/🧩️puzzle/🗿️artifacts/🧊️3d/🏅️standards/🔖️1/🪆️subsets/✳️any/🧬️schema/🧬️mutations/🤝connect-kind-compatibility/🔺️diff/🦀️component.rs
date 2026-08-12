//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — patches the document `meta.kindCompatibility`.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::{Puzzle3dKindCompatibility, Puzzle3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectKindCompatibility, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    if base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return Puzzle3dDiff::default();
    }
    let mut meta = base.meta.clone();
    meta.kind_compatibility.push(Puzzle3dKindCompatibility {
        source: payload.source.clone(),
        target: payload.target.clone(),
        bidirectional: payload.bidirectional,
        important: payload.important,
        specificity: payload.specificity,
    });
    Puzzle3dDiff { meta: Some(meta), ..Default::default() }
}
//#endregion 🔖️Diff
