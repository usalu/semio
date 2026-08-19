//! ↩️ `delete-column` — undo re-creates the column at its original BASE index, then patches every
//! row's cell at that column back to its original BASE value (in row order): a bare re-create only
//! fills `Null`, so the full per-row cascade must be recaptured explicitly, per
//! `📓️taxonomy.md`'s `delete` row ("captures cascade"). Column absent from `base` ⇒ `Vec::new()`.

use super::mutation::DeleteColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{create_column, edit_cell};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteColumn, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return Vec::new();
    };
    let kind = base.columns[at].kind;

    let mut mutations = vec![SemioTableMutation::CreateColumn(create_column::mutation::CreateColumn { name: payload.name.clone(), kind, index: Some(at) })];
    for (row_index, row) in base.rows.iter().enumerate() {
        if let Some(cell) = row.cells.get(at) {
            mutations.push(SemioTableMutation::EditCell(edit_cell::mutation::EditCell { row_index, column_name: payload.name.clone(), new_value: cell.clone() }));
        }
    }
    mutations
}
//#endregion 🔖️Inverse
