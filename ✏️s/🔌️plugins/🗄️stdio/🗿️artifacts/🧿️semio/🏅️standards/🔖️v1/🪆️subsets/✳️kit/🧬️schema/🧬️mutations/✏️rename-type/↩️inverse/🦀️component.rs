//! ↩️ `rename-type` — self-inverse: another rename restoring the BASE-state name; empty when
//! the id was absent.

use super::mutation::RenameType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &RenameType, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.types.iter().find(|t| t.id == payload.id) {
        Some(existing) => vec![SemioKitMutation::RenameType(RenameType { id: payload.id.clone(), new_name: existing.name.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
