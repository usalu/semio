//! ↩️ Inverse for `RemoveRow`.

use crate::schema::mutations::{insert_row, SemioTableMutation};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveRow, base: &SemioTableSnapshot) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.rows.get(payload.index) {
        Some(row) => vec![SemioTableMutation::InsertRow(insert_row::InsertRow { index: payload.index, row: row.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
