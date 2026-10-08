//! ↩️ Inverse for `InsertStyle`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertStyle, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::InsertStyle { style } = payload;
    Ok(vec![SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: style.id.clone() })])
}
//#endregion 🔖️Inverse
