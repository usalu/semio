//! ↩️ Inverse for `RenameType`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RenameType, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.types.iter().find(|t| t.id == payload.id) {
        Some(existing) => vec![SemioKitMutation::RenameType(super::RenameType { id: payload.id.clone(), new_name: existing.name.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
