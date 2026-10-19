//! ↩️ Inverse for `ReorderColumns`.

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioTableOp>(payload: &super::ReorderColumns, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(from) = base.columns.iter().position(|c| c.name == payload.name) else {
        return Vec::new();
    };
    vec![Op::from(super::ReorderColumns { name: payload.name.clone(), to_index: from })]

    })())
}
//#endregion 🔖️Inverse
