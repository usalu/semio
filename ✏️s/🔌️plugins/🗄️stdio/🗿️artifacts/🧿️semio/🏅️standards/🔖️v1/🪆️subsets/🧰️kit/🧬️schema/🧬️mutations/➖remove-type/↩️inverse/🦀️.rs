//! ↩️ Inverse for `RemoveType`.

use crate::schema::mutations::add_type;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveType, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::add_type::AddType>,
{
    Ok((|| {
    match base.types.iter().position(|t| t.id == payload.id) {
        Some(at) => vec![Op::from(add_type::AddType { id: base.types[at].id.clone(), name: base.types[at].name.clone(), category: base.types[at].category.clone(), at: Some(at) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
