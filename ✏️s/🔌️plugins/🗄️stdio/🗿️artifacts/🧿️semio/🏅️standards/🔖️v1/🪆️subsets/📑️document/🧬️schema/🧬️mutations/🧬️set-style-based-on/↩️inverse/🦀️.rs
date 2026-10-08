//! ↩️ Inverse for `SetStyleBasedOn`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetStyleBasedOn, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetStyleBasedOn { id, .. } = payload;
    Ok(match style_at(base, id) {
        Some(style) => vec![SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
