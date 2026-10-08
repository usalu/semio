//! ↩️ Inverse for `CreateColumn`.

use crate::standards::v1::subsets::table::schema::mutations::{delete_column, SemioTableMutation};
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

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
