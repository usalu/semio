//! 🔺️ Diff for `InsertRow`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::schema::diff::SemioTableDiff;
use crate::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
/// 🧮️ Inserts one row at `index` (clamped to the end): the `rows` triple names exactly that addition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertRow, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let at = payload.index.min(base.rows.len());
    let outcome = protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(IndexedTripleDiff { added: vec![IndexAdded { index: at, item: payload.row.clone() }], ..Default::default() }) });
    if at == payload.index {
        outcome
    } else {
        outcome.warning("mutation.clamped", format!("Insert index {} was out of range; inserted at #{} instead.", payload.index, at))
    }
}
//#endregion 🔖️Diff
