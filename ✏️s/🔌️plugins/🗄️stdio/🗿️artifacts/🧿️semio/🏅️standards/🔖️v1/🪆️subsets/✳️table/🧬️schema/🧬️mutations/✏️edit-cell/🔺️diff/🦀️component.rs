//! 🔺️ `edit-cell` — sparse diff construction; Error `mutation.target-missing` when the addressed
//! row or column doesn't exist, Warning `mutation.no-op` when the new value already equals the
//! current cell value. Columns untouched — an edit is a pure cell-value change.

use super::mutation::EditCell;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditCell, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
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
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cell #{} {} already has this value.", payload.row_index, payload.column_name));
    }
    let mut rows = base.rows.clone();
    rows[payload.row_index].cells[col_index] = payload.new_value.clone();
    protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) })
}
//#endregion 🔖️Diff
