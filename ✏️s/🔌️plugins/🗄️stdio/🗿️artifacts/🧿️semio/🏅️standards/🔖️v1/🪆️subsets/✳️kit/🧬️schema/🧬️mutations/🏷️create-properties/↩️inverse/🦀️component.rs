//! ↩️ `create-properties` — undo restores whichever handle occupied `properties` BEFORE this
//! create ran.

use super::mutation::CreateProperties;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{delete_properties, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &CreateProperties, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match &base.properties {
        Some(existing) => vec![SemioKitMutation::CreateProperties(CreateProperties { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![SemioKitMutation::DeleteProperties(delete_properties::mutation::DeleteProperties {})],
    }
}
//#endregion 🔖️Inverse
