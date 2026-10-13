//! 🔺️ Diff for `RemoveRun`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::SemioTextDiff;
use crate::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Removes the run at `index`: the `runs` triple names exactly that removal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    if payload.index >= base.runs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { removed: vec![payload.index], ..Default::default() }) })
}
//#endregion 🔖️Diff
