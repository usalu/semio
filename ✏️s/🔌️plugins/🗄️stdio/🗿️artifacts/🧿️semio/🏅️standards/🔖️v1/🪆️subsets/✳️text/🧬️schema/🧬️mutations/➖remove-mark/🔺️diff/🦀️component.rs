//! 🔺️ `remove-mark` — sparse diff construction; an out-of-range BASE `run_index` or a mark
//! `index` not present on that run is `mutation.target-missing`.

use super::mutation::RemoveMark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &RemoveMark, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.run_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.run_index), [payload.run_index.to_string()]);
    };
    if payload.index >= existing.marks.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mark #{} does not exist on run #{}.", payload.index, payload.run_index), [payload.run_index.to_string(), payload.index.to_string()]);
    }
    let mut runs = base.runs.clone();
    runs[payload.run_index].marks.remove(payload.index);
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) })
}
//#endregion 🔖️Diff
