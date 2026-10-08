//! 🔺️ Diff for `InsertRun`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunDiff};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Inserts one run at `index` (clamped to the end): the `runs` triple names exactly that addition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let at = payload.index.min(base.runs.len());
    let outcome = protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { added: vec![IndexAdded { index: at, item: payload.run.clone() }], ..Default::default() }) });
    if at != payload.index {
        outcome.warning("mutation.clamped", format!("Insert index {} was out of range; inserted at #{} instead.", payload.index, at))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
