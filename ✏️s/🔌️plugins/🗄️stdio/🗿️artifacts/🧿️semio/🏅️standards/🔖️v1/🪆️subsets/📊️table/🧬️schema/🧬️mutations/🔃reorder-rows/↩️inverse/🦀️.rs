//! ↩️ Inverse for `ReorderRows`.

use crate::schema::mutations::SemioTableOp;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioTableOp>(payload: &super::ReorderRows, base: &SemioTableSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let len = base.rows.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![Op::from(super::ReorderRows { from: landed_at, to: payload.from })]

    })())
}
//#endregion 🔖️Inverse
