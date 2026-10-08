//! 🔺️ Diff for `AddMark`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunDiff};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Inserts one mark at `index` (clamped to the end): the run's `marks` triple names exactly that addition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddMark, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.run_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.run_index), [payload.run_index.to_string()]);
    };
    if existing.marks.contains(&payload.mark) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Run #{} already has mark {:?}.", payload.run_index, payload.mark));
    }
    let at = payload.index.min(existing.marks.len());
    let marks = IndexedTripleDiff { added: vec![IndexAdded { index: at, item: payload.mark.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { modified: vec![IndexModified { index: payload.run_index, diff: SemioTextRunDiff { marks: Some(marks), ..Default::default() } }], ..Default::default() }) })
}
//#endregion 🔖️Diff
