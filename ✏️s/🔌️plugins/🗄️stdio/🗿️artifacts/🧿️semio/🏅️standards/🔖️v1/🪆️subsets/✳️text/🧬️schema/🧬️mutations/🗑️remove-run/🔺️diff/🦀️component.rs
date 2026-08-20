//! 🔺️ `remove-run` — sparse diff construction; an out-of-range BASE index is `mutation.target-missing`.

use super::mutation::RemoveRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &RemoveRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    if payload.index >= base.runs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.index), [payload.index.to_string()]);
    }
    let mut runs = base.runs.clone();
    runs.remove(payload.index);
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) })
}
//#endregion 🔖️Diff
