//! ↩️ Inverse for `DeleteColumn`.

use crate::schema::mutations::{SemioTableOp, create_column, edit_cell};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioTableOp>(payload: &super::DeleteColumn, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(at) = base.columns.iter().position(|c| c.name == payload.name) else {
        return Vec::new();
    };
    let kind = base.columns[at].kind;

    let mut mutations = vec![Op::from(create_column::CreateColumn { name: payload.name.clone(), kind, index: Some(at) })];
    for (row_index, row) in base.rows.iter().enumerate() {
        if let Some(cell) = row.cells.get(at) {
            mutations.push(Op::from(edit_cell::EditCell { row_index, column_name: payload.name.clone(), new_value: cell.clone() }));
        }
    }
    mutations.reverse();
    mutations

    })())
}
//#endregion 🔖️Inverse
