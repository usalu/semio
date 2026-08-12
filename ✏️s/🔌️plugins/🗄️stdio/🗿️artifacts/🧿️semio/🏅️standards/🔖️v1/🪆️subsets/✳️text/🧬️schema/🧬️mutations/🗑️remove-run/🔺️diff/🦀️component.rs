//! 🔺️ `remove-run` — sparse diff construction; an out-of-range BASE index is a no-op clone.

use super::mutation::RemoveRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveRun, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    if payload.index < runs.len() {
        runs.remove(payload.index);
    }
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
