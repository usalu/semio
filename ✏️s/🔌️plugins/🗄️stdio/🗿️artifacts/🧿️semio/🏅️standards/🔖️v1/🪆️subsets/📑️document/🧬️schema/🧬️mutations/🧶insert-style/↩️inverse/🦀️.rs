//! ↩️ Inverse for `InsertStyle`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_style;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertStyle, _base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_style::RemoveStyle>,
{
    let super::InsertStyle { style, .. } = payload;
    Ok(vec![Op::from(remove_style::RemoveStyle { id: style.id.clone() })])
}
//#endregion 🔖️Inverse
