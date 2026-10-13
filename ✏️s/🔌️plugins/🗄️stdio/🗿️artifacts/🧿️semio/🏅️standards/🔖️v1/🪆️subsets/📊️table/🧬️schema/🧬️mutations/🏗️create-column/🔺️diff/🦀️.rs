//! 🔺️ Diff for `CreateColumn`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::schema::diff::{SemioTableDiff, SemioTableRowDiff};
use crate::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValue;

//#region 🔖️Diff
/// 🧮️ Inserts one column at `index` (clamped to the end) and a `Null` cell at the same position of every row.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    if base.columns.iter().any(|c| c.name == payload.name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A column named \"{}\" already exists.", payload.name), [payload.name.clone()]);
    }
    let at = payload.index.unwrap_or(base.columns.len()).min(base.columns.len());
    let columns = IndexedTripleDiff { added: vec![IndexAdded { index: at, item: SemioTableColumn { name: payload.name.clone(), kind: payload.kind } }], ..Default::default() };
    let rows = IndexedTripleDiff {
        modified: base
            .rows
            .iter()
            .enumerate()
            .map(|(index, row)| IndexModified { index, diff: SemioTableRowDiff { cells: Some(IndexedTripleDiff { added: vec![IndexAdded { index: at.min(row.cells.len()), item: SemioValue::Null }], ..Default::default() }) } })
            .collect(),
        ..Default::default()
    };
    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(columns), rows: (!rows.modified.is_empty()).then_some(rows) })
}
//#endregion 🔖️Diff
