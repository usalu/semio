//! ↩️ Inverse for `DeleteProperties`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, create_properties};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::DeleteProperties, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match &base.properties {
        Some(existing) => vec![SemioKitMutation::CreateProperties(create_properties::CreateProperties { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
