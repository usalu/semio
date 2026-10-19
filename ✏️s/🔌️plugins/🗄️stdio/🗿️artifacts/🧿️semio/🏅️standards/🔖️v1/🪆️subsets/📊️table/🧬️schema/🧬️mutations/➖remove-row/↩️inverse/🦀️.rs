//! ↩️ Inverse for `RemoveRow`.

use crate::schema::mutations::{SemioTableOp, insert_row};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioTableOp>(payload: &super::RemoveRow, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.rows.get(payload.index) {
        Some(row) => vec![Op::from(insert_row::InsertRow { index: payload.index, row: row.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
