//! ↩️ Inverse for `CreateColumn`.

use crate::schema::mutations::{delete_column, SemioTableMutation};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateColumn, base: &SemioTableSnapshot) -> Result<Vec<SemioTableMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.columns.iter().any(|c| c.name == payload.name) {
        return Vec::new();
    }
    vec![SemioTableMutation::DeleteColumn(delete_column::DeleteColumn { name: payload.name.clone() })]

    })())
}
//#endregion 🔖️Inverse
