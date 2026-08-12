//! 🔺️ `edit-run` — sparse diff construction; an out-of-range BASE index is a no-op clone.

use super::mutation::EditRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditRun, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    if let Some(run) = runs.get_mut(payload.index) {
        run.content = payload.new_content.clone();
    }
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
