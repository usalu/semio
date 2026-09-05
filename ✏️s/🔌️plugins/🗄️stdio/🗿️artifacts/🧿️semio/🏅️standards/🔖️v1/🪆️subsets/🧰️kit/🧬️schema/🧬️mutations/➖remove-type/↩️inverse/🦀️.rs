//! ↩️ Inverse for `RemoveType`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, add_type};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveType, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.types.iter().find(|t| t.id == payload.id) {
        Some(existing) => vec![SemioKitMutation::AddType(add_type::AddType { id: existing.id.clone(), name: existing.name.clone(), category: existing.category.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
