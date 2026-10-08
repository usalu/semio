//! ↩️ Inverse for `RemoveElement`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveElement, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::RemoveElement { id } = payload;
    Ok(match base.elements.iter().find(|e| &e.id == id) {
        Some(original) => vec![SemioModelMutation::InsertElement(insert_element::InsertElement { element: original.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
