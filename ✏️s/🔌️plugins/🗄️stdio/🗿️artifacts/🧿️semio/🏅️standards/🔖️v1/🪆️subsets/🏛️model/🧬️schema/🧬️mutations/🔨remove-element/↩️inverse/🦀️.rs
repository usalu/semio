//! ↩️ Inverse for `RemoveElement`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveElement, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::RemoveElement { id } = payload;
    Ok(match base.elements.iter().position(|e| &e.id == id) {
        Some(at) => vec![SemioModelMutation::InsertElement(insert_element::InsertElement { element: base.elements[at].clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
