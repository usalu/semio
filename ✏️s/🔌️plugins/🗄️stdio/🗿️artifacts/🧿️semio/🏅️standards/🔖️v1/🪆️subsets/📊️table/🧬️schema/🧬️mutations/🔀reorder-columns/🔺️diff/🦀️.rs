//! 🔺️ Diff for `ReorderColumns`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::schema::diff::{SemioTableColumnDiff, SemioTableDiff, SemioTableRowDiff};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

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
