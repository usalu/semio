//! 🔺️ `reorder-runs` — sparse diff construction; an out-of-range BASE `from` is
//! `mutation.target-missing`, and `from == to` (already in place) is `mutation.no-op`.

use super::mutation::ReorderRuns;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &ReorderRuns, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    if payload.from >= base.runs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.from), [payload.from.to_string()]);
    }
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Run #{} is already at position #{}.", payload.from, payload.to));
    }
    let mut runs = base.runs.clone();
    let item = runs.remove(payload.from);
    let at = payload.to.min(runs.len());
    runs.insert(at, item);
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) })
}
//#endregion 🔖️Diff
