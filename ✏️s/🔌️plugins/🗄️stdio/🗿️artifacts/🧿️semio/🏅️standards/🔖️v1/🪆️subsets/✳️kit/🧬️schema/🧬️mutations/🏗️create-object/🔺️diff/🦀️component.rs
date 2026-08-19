//! 🔺️ `create-object` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateObject;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitObjectChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateObject, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.objects.iter().any(|o| o.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An object child with id \"{}\" already exists.", payload.child_id),
            [payload.child_id.clone()],
        );
    }
    let mut objects = base.objects.clone();
    objects.push(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()));
    protocol::MutationOutcome::new(SemioKitDiff { objects: Some(SemioKitObjectChildList { values: objects }), ..Default::default() })
}
//#endregion 🔖️Diff
