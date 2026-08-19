//! 🗑️ `delete-mesh` — Error `mutation.target-missing` when mesh `id` is absent.

use super::mutation::DeleteMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteMesh, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::mesh_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mesh \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_mesh(base, &payload.id))
}
//#endregion 🔖️Diff
