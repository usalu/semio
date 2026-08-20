//! 🕸️ `create-mesh` — Fatal `mutation.duplicate-id` when mesh `id` already exists.

use super::mutation::CreateMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &CreateMesh, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::mesh_at(base, &payload.mesh.id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Mesh \"{}\" already exists.", payload.mesh.id), [payload.mesh.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_mesh(base, payload.mesh.clone()))
}
//#endregion 🔖️Diff
