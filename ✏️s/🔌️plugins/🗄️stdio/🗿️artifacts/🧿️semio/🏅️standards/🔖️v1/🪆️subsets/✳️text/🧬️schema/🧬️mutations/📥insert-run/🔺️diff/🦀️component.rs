//! 🔺️ `insert-run` — sparse diff construction. `SemioTextDiff::runs` is a whole-list-per-diff
//! wrapper (`SemioTextRunList`), not a sparse triple — every run mutation rebuilds the full
//! ordered `values` vec from `base` and wraps it. An out-of-range index clamps to the end with
//! `mutation.clamped`.

use super::mutation::InsertRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &InsertRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let mut runs = base.runs.clone();
    let at = payload.index.min(runs.len());
    runs.insert(at, payload.run.clone());
    let outcome = protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }).await;
    if at != payload.index {
        outcome.warn("mutation.clamped", format!("Insert index {} was out of range; inserted at #{} instead.", payload.index, at)).await
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
