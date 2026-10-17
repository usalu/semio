//! ↩️ Inverse for `AddDesign`.

use crate::schema::mutations::remove_design;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddDesign, _base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_design::RemoveDesign>,
{
    Ok((|| {
    vec![Op::from(remove_design::RemoveDesign { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
