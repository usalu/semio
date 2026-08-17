//! 🔺️ `create-mesh` — sparse diff construction, built directly from `(payload, base)`. A `mesh`
//! slot already occupied in `base` is `mutation.duplicate-id` (Fatal, empty diff — a true "create"
//! never silently overwrites).

use super::mutation::CreateMesh;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMesh, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.mesh.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a mesh child.".to_string(), ["mesh".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
