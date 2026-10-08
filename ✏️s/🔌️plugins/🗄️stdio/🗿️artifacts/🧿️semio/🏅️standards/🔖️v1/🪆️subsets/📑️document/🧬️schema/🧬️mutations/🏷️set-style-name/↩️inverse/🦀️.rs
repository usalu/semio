//! ↩️ Inverse for `SetStyleName`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetStyleName, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetStyleName { id, .. } = payload;
    Ok(match style_at(base, id) {
        Some(style) => vec![SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: id.clone(), name: style.name.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
