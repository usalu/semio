//! ↩️ Inverse for `InsertElement`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::remove_element;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertElement, _base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_element::RemoveElement>,
{
    let super::InsertElement { element, .. } = payload;
    Ok(vec![Op::from(remove_element::RemoveElement { id: element.id.clone() })])
}
//#endregion 🔖️Inverse
