//! ↩️ Inverse for `InsertRow`.

use crate::schema::mutations::{remove_row, SemioTableMutation};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertRow, base: &SemioTableSnapshot) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.min(base.rows.len());
    vec![SemioTableMutation::RemoveRow(remove_row::RemoveRow { index: at })]

    })())
}
//#endregion 🔖️Inverse
