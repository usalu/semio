//! ↩️ Inverse for `DeleteColumn`.

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{SemioTableMutation, create_column, edit_cell};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteColumn, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return Vec::new();
    };
    let kind = base.columns[at].kind;

    let mut mutations = vec![SemioTableMutation::CreateColumn(create_column::CreateColumn { name: payload.name.clone(), kind, index: Some(at) })];
    for (row_index, row) in base.rows.iter().enumerate() {
        if let Some(cell) = row.cells.get(at) {
            mutations.push(SemioTableMutation::EditCell(edit_cell::EditCell { row_index, column_name: payload.name.clone(), new_value: cell.clone() }));
        }
    }
    mutations
}
//#endregion 🔖️Inverse
