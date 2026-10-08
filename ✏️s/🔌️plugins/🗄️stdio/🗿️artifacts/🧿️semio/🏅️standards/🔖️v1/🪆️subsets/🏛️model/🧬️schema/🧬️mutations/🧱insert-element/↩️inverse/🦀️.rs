//! ↩️ Inverse for `InsertElement`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertElement, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::InsertElement { element, .. } = payload;
    Ok(vec![SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: element.id.clone() })])
}
//#endregion 🔖️Inverse
