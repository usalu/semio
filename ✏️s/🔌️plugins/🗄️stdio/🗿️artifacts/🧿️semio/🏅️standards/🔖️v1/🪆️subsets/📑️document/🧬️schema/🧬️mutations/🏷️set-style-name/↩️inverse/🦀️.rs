//! ↩️ Inverse for `SetStyleName`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_style_name;
#[allow(unused_imports)]
use crate::schema::mutations::support::style_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetStyleName, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_style_name::SetStyleName>,
{
    let super::SetStyleName { id, .. } = payload;
    Ok(match style_at(base, id) {
        Some(style) => vec![Op::from(set_style_name::SetStyleName { id: id.clone(), name: style.name.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
