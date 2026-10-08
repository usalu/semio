//! 🔺️ Diff for `EditCell`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::schema::diff::{SemioTableColumnDiff, SemioTableDiff, SemioTableRowDiff};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Diff
/// 🧮️ Replaces one cell: the row's `cells` triple names exactly that cell's new value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::EditCell, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(col_index) = base.columns.iter().position(|c| c.name == payload.column_name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.column_name), [payload.column_name.clone()]);
    };
    let Some(row) = base.rows.get(payload.row_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Row #{} does not exist.", payload.row_index), [payload.row_index.to_string()]);
    };
    let Some(current) = row.cells.get(col_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Row #{} has no cell for column \"{}\".", payload.row_index, payload.column_name), [payload.row_index.to_string(), payload.column_name.clone()]);
    };
    if *current == payload.new_value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Cell #{} {} already has this value.", payload.row_index, payload.column_name));
    }
    let cells = IndexedTripleDiff { modified: vec![IndexModified { index: col_index, diff: Replace { value: payload.new_value.clone() } }], ..Default::default() };
    protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(IndexedTripleDiff { modified: vec![IndexModified { index: payload.row_index, diff: SemioTableRowDiff { cells: Some(cells) } }], ..Default::default() }) })
}
//#endregion 🔖️Diff
