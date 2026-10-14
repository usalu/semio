//! ↩️ Inverse for `EditCell`.

use crate::schema::mutations::SemioTableMutation;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::EditCell, base: &SemioTableSnapshot) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(col_index) = base.columns.iter().position(|c| c.name == payload.column_name) else {
        return Vec::new();
    };
    let Some(row) = base.rows.get(payload.row_index) else {
        return Vec::new();
    };
    let Some(cell) = row.cells.get(col_index) else {
        return Vec::new();
    };
    vec![SemioTableMutation::EditCell(super::EditCell { row_index: payload.row_index, column_name: payload.column_name.clone(), new_value: cell.clone() })]

    })())
}
//#endregion 🔖️Inverse
