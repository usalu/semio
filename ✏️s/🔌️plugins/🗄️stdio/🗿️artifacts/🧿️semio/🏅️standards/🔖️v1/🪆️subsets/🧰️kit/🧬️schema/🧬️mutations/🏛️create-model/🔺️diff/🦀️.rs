//! 🔺️ Diff for `CreateModel`.

use crate::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitModelChildList};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateModel, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.models.iter().any(|m| m.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A model child with id \"{}\" already exists.", payload.child_id), [payload.child_id.clone()]);
    }
    if let Err(message) = crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "model") {
        return protocol::MutationOutcome::fatal("mutation.child-identity", message, ["models".to_string()]);
    }
    let mut models = base.models.clone();
    models.push(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()));
    protocol::MutationOutcome::new(SemioKitDiff { models: Some(SemioKitModelChildList { values: models }), ..Default::default() })
}
//#endregion 🔖️Diff
