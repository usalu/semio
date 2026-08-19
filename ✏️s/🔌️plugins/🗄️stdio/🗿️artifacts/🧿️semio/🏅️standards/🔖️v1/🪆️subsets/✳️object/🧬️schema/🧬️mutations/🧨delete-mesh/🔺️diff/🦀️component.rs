//! 🔺️ `delete-mesh` — sparse diff construction: clears `mesh`. An already-empty `mesh` slot is
//! `mutation.target-missing` (Error, empty diff — nothing to delete).

use super::mutation::DeleteMesh;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub async fn diff(_payload: &DeleteMesh, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.mesh.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The object has no mesh to delete.".to_string(), ["mesh".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { mesh: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
