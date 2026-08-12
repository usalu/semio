//! ↩️ `edit-cell` — undo restores BASE's cell value at that `row_index`/`column_name`; a missing
//! row or column in `base` ⇒ `Vec::new()`.

use super::mutation::EditCell;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditCell, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let Some(col_index) = base.columns.iter().position(|c| c.name == payload.column_name) else {
        return Vec::new();
    };
    let Some(row) = base.rows.get(payload.row_index) else {
        return Vec::new();
    };
    let Some(cell) = row.cells.get(col_index) else {
        return Vec::new();
    };
    vec![SemioTableMutation::EditCell(EditCell { row_index: payload.row_index, column_name: payload.column_name.clone(), new_value: cell.clone() })]
}
//#endregion 🔖️Inverse
