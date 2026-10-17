//! ↩️ Inverse for `RemoveElement`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::insert_element;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveElement, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_element::InsertElement>,
{
    let super::RemoveElement { id } = payload;
    Ok(match base.elements.iter().position(|e| &e.id == id) {
        Some(at) => vec![Op::from(insert_element::InsertElement { element: base.elements[at].clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
