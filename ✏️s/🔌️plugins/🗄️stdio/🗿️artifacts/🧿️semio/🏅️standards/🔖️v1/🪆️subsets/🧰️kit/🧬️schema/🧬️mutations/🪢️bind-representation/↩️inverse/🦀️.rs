//! ↩️ Inverse for `BindRepresentation`.

use crate::schema::mutations::unbind_representation;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::BindRepresentation, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::unbind_representation::UnbindRepresentation>,
{
    Ok((|| {
    vec![Op::from(unbind_representation::UnbindRepresentation { index: base.representations.len() })]

    })())
}
//#endregion 🔖️Inverse
