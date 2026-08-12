//! ↩️ `create-model` — undo is `delete-model` for the just-minted `child_id`.

use super::mutation::CreateModel;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{delete_model, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateModel, _base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    vec![SemioKitMutation::DeleteModel(delete_model::mutation::DeleteModel { child_id: payload.child_id.clone() })]
}
//#endregion 🔖️Inverse
