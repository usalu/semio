//! 🔺️ `reorder-runs` — sparse diff construction; an out-of-range BASE `from` is a no-op clone.

use super::mutation::ReorderRuns;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReorderRuns, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    if payload.from < runs.len() {
        let item = runs.remove(payload.from);
        let at = payload.to.min(runs.len());
        runs.insert(at, item);
    }
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
