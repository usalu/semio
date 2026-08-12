//! 🔺️ `insert-run` — sparse diff construction. `SemioTextDiff::runs` is a whole-list-per-diff
//! wrapper (`SemioTextRunList`), not a sparse triple — every run mutation rebuilds the full
//! ordered `values` vec from `base` and wraps it.

use super::mutation::InsertRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &InsertRun, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    let at = payload.index.min(runs.len());
    runs.insert(at, payload.run.clone());
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
