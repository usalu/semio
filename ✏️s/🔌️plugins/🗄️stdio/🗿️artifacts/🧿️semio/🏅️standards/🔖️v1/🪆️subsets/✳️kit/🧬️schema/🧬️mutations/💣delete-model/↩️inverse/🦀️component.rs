//! ↩️ `delete-model` — undo is `create-model` with the escrowed handle from BASE; empty when
//! absent.

use super::mutation::DeleteModel;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{create_model, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteModel, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.models.iter().find(|c| c.child_id == payload.child_id) {
        Some(existing) => vec![SemioKitMutation::CreateModel(create_model::mutation::CreateModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
