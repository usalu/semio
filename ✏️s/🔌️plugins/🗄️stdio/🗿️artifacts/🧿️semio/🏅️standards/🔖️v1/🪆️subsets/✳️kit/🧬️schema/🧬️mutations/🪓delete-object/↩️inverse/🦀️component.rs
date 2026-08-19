//! ↩️ `delete-object` — undo is `create-object` with the escrowed handle from BASE; empty when
//! the id was already absent (nothing to undo).

use super::mutation::DeleteObject;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{create_object, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteObject, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.objects.iter().find(|c| c.child_id == payload.child_id) {
        Some(existing) => vec![SemioKitMutation::CreateObject(create_object::mutation::CreateObject { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
