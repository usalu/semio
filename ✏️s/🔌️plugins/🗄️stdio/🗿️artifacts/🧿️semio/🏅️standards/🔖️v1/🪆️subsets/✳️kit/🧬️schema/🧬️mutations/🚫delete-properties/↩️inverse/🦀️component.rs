//! ↩️ `delete-properties` — undo is `create-properties` with the escrowed handle from BASE; empty
//! when absent.

use super::mutation::DeleteProperties;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{create_properties, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &DeleteProperties, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match &base.properties {
        Some(existing) => vec![SemioKitMutation::CreateProperties(create_properties::mutation::CreateProperties { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
