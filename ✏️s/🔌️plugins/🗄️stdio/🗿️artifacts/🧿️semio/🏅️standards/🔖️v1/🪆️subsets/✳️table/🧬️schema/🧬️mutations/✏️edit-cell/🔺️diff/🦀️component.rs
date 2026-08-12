//! 🔺️ `edit-cell` — sparse diff construction; a missing row or column is a no-op clone (rows
//! unchanged). Columns untouched — an edit is a pure cell-value change.

use super::mutation::EditCell;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditCell, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut rows = base.rows.clone();
    if let Some(col_index) = base.columns.iter().position(|c| c.name == payload.column_name) {
        if let Some(row) = rows.get_mut(payload.row_index) {
            if col_index < row.cells.len() {
                row.cells[col_index] = payload.new_value.clone();
            }
        }
    }
    SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
