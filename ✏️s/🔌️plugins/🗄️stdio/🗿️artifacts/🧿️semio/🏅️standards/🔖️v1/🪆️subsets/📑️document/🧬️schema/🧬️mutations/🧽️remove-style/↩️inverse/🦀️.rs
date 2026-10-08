//! ↩️ Inverse for `RemoveStyle`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveStyle, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::RemoveStyle { id } = payload;
    Ok(match style_at(base, id) {
        Some(style) => vec![SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: style.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
