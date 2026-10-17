//! ↩️ Inverse for `SetStyleBasedOn`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_style_based_on;
#[allow(unused_imports)]
use crate::schema::mutations::support::style_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetStyleBasedOn, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_style_based_on::SetStyleBasedOn>,
{
    let super::SetStyleBasedOn { id, .. } = payload;
    Ok(match style_at(base, id) {
        Some(style) => vec![Op::from(set_style_based_on::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
