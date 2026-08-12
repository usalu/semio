//! 🔺️ `remove-mark` — sparse diff construction; an out-of-range BASE `run_index`/`index` is a
//! no-op clone.

use super::mutation::RemoveMark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveMark, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    if let Some(run) = runs.get_mut(payload.run_index) {
        if payload.index < run.marks.len() {
            run.marks.remove(payload.index);
        }
    }
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
