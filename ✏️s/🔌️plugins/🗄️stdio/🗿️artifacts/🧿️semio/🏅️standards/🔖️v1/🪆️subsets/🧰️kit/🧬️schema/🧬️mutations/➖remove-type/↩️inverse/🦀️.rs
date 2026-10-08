//! ↩️ Inverse for `RemoveType`.

use crate::standards::v1::subsets::kit::schema::mutations::{add_type, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveType, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.types.iter().position(|t| t.id == payload.id) {
        Some(at) => vec![SemioKitMutation::AddType(add_type::AddType { id: base.types[at].id.clone(), name: base.types[at].name.clone(), category: base.types[at].category.clone(), at: Some(at) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
