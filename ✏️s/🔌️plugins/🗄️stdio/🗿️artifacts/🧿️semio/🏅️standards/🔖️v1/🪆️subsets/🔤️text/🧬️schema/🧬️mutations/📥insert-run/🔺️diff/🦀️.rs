//! 🔺️ Diff for `InsertRun`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let mut runs = base.runs.clone();
    let at = payload.index.min(runs.len());
    runs.insert(at, payload.run.clone());
    let outcome = protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) });
    if at != payload.index {
        outcome.warn("mutation.clamped", format!("Insert index {} was out of range; inserted at #{} instead.", payload.index, at))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
