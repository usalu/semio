//! ↩️ `delete-properties` — undo is `create-properties` with the escrowed handle from BASE; empty
//! when absent.

use super::mutation::DeleteProperties;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{create_properties, SemioObjectMutation};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &DeleteProperties, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.properties {
        Some(existing) => vec![SemioObjectMutation::CreateProperties(create_properties::mutation::CreateProperties { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
