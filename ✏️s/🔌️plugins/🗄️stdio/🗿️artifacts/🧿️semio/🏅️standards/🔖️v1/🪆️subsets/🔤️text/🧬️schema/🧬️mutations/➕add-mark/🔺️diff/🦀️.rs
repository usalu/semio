//! 🔺️ Diff for `AddMark`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddMark, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.run_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.run_index), [payload.run_index.to_string()]);
    };
    if existing.marks.contains(&payload.mark) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Run #{} already has mark {:?}.", payload.run_index, payload.mark));
    }
    let mut runs = base.runs.clone();
    let run = &mut runs[payload.run_index];
    let at = payload.index.min(run.marks.len());
    run.marks.insert(at, payload.mark.clone());
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) })
}
//#endregion 🔖️Diff
