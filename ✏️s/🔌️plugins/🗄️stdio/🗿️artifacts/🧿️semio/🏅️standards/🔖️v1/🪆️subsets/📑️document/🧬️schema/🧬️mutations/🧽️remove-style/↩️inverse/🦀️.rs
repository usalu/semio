//! ↩️ Inverse for `RemoveStyle`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_style;
#[allow(unused_imports)]
use crate::schema::mutations::support::style_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveStyle, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_style::InsertStyle>,
{
    let super::RemoveStyle { id } = payload;
    Ok(match style_at(base, id) {
        Some(style) => vec![Op::from(insert_style::InsertStyle { style: style.clone(), at: base.styles.iter().position(|s| s.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
