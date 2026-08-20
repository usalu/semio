//! 🔺️ `create-model` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateModel;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitModelChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateModel, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.models.iter().any(|m| m.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("A model child with id \"{}\" already exists.", payload.child_id),
            [payload.child_id.clone()],
        ).await;
    }
    let mut models = base.models.clone();
    models.push(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()).await);
    protocol::MutationOutcome::new(SemioKitDiff { models: Some(SemioKitModelChildList { values: models }), ..Default::default() }).await
}
//#endregion 🔖️Diff
