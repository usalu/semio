//! 🔺️ Sparse diff builder for `ConnectKindCompatibility` — patches the whole `kindCompatibility` list.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dKindCompatibilityList};
use crate::artifacts::puzzle5d::{Puzzle5dKindCompatibility, Puzzle5dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectKindCompatibility, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    if base.kind_compatibility.iter().any(|row| row.source == payload.source && row.target == payload.target) {
        return Puzzle5dDiff::default();
    }
    let mut values = base.kind_compatibility.clone();
    values.push(Puzzle5dKindCompatibility {
        source: payload.source.clone(),
        target: payload.target.clone(),
        bidirectional: payload.bidirectional,
        important: payload.important,
        specificity: payload.specificity,
    });
    Puzzle5dDiff { kind_compatibility: Some(Puzzle5dKindCompatibilityList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
