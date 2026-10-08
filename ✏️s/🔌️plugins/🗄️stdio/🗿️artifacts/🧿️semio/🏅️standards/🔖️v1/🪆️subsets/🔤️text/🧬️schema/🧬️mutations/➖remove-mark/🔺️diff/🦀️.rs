//! 🔺️ Diff for `RemoveMark`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunDiff};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Removes the mark at `index`: the run's `marks` triple names exactly that removal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveMark, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.run_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.run_index), [payload.run_index.to_string()]);
    };
    if payload.index >= existing.marks.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mark #{} does not exist on run #{}.", payload.index, payload.run_index), [payload.run_index.to_string(), payload.index.to_string()]);
    }
    let marks = IndexedTripleDiff { removed: vec![payload.index], ..Default::default() };
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { modified: vec![IndexModified { index: payload.run_index, diff: SemioTextRunDiff { marks: Some(marks), ..Default::default() } }], ..Default::default() }) })
}
//#endregion 🔖️Diff
