//! 🔺️ `add-mark` — sparse diff construction; an out-of-range BASE `run_index` is a no-op clone
//! (nothing at that position to attach a mark to).

use super::mutation::AddMark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &AddMark, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    if let Some(run) = runs.get_mut(payload.run_index) {
        let at = payload.index.min(run.marks.len());
        run.marks.insert(at, payload.mark.clone());
    }
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
