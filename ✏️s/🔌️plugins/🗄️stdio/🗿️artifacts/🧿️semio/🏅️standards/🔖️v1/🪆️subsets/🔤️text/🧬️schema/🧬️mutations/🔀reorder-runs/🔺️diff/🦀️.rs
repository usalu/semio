//! 🔺️ Diff for `ReorderRuns`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunDiff};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Moves one run: the `runs` triple removes its base index and re-adds the same run at its landing position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReorderRuns, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(run) = base.runs.get(payload.from) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.from), [payload.from.to_string()]);
    };
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Run #{} is already at position #{}.", payload.from, payload.to));
    }
    let at = payload.to.min(base.runs.len() - 1);
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { removed: vec![payload.from], added: vec![IndexAdded { index: at, item: run.clone() }], ..Default::default() }) })
}
//#endregion 🔖️Diff
