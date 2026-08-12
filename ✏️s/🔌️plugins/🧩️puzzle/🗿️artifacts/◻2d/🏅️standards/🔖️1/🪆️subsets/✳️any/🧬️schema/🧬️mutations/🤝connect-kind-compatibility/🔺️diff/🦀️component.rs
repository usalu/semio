//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — patches `meta.kindCompatibility`.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::{Puzzle2dKindCompatibility, Puzzle2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectKindCompatibility, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
    if base.meta.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return Puzzle2dDiff::default();
    }
    let mut meta = base.meta.clone();
    meta.kind_compatibility.push(Puzzle2dKindCompatibility {
        source: payload.source.clone(),
        target: payload.target.clone(),
        bidirectional: payload.bidirectional,
        important: payload.important,
        specificity: payload.specificity,
    });
    Puzzle2dDiff { meta: Some(meta), ..Default::default() }
}
//#endregion 🔖️Diff
