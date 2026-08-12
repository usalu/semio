//! ↩️ `create-mesh` — undo is `delete-mesh` at the same id (unconditional: if the create was
//! itself a no-op duplicate, the delete's own presence check makes this a no-op too).

use super::mutation::CreateMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{delete_mesh, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateMesh, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeleteMesh(delete_mesh::mutation::DeleteMesh { id: payload.mesh.id.clone() })]
}
//#endregion 🔖️Inverse
