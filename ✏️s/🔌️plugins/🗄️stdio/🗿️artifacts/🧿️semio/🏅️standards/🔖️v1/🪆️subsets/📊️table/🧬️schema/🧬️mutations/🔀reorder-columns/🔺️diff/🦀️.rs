//! 🔺️ Diff for `ReorderColumns`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff;
use semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableRowDiff;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
/// 🧮️ Moves one column: the `columns` triple removes its base index and re-adds it at the landing position, and every row moves the same cell.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReorderColumns, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(from) = base.columns.iter().position(|c| c.name == payload.name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    };
    if from == payload.to_index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Column \"{}\" is already at position #{}.", payload.name, payload.to_index));
    }
    let to = payload.to_index.min(base.columns.len() - 1);
    let columns = IndexedTripleDiff { removed: vec![from], added: vec![IndexAdded { index: to, item: base.columns[from].clone() }], ..Default::default() };
    let rows = IndexedTripleDiff {
        modified: base
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| from < row.cells.len())
            .map(|(index, row)| IndexModified { index, diff: SemioTableRowDiff { cells: Some(IndexedTripleDiff { removed: vec![from], added: vec![IndexAdded { index: to.min(row.cells.len() - 1), item: row.cells[from].clone() }], ..Default::default() }) } })
            .collect(),
        ..Default::default()
    };
    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(columns), rows: (!rows.modified.is_empty()).then_some(rows) })
}
//#endregion 🔖️Diff
