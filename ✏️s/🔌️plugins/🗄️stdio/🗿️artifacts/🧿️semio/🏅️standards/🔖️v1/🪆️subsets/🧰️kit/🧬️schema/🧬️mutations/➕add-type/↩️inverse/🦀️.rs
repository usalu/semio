//! ↩️ Inverse for `AddType`.

use crate::schema::mutations::remove_type;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddType, _base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_type::RemoveType>,
{
    Ok((|| {
    vec![Op::from(remove_type::RemoveType { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
