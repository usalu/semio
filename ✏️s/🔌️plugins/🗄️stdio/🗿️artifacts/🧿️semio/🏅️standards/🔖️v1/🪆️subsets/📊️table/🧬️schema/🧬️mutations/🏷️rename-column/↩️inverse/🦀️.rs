//! ↩️ Inverse for `RenameColumn`.

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioTableOp>(payload: &super::RenameColumn, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if !base.columns.iter().any(|c| c.name == payload.name) {
        return Vec::new();
    }
    vec![Op::from(super::RenameColumn { name: payload.new_name.clone(), new_name: payload.name.clone() })]

    })())
}
//#endregion 🔖️Inverse
