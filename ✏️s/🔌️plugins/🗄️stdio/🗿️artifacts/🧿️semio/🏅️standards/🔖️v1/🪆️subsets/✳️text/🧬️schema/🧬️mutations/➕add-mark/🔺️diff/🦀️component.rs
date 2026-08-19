//! 🔺️ `add-mark` — sparse diff construction; an out-of-range BASE `run_index` is
//! `mutation.target-missing` (nothing at that position to attach a mark to), and a mark already
//! present on the run is `mutation.no-op`.

use super::mutation::AddMark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &AddMark, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
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
