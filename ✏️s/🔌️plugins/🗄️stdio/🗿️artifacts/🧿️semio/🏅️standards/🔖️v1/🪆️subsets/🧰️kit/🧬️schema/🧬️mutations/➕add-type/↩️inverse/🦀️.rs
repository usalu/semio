//! ↩️ Inverse for `AddType`.

use crate::schema::mutations::{remove_type, SemioKitMutation};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddType, _base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioKitMutation::RemoveType(remove_type::RemoveType { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
