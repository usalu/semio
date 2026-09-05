//! ↩️ Inverse for `DeleteObject`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, create_object};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteObject, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.objects.iter().find(|c| c.child_id == payload.child_id) {
        Some(existing) => vec![SemioKitMutation::CreateObject(create_object::CreateObject { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
