//! ↩️ `create-material` — undo is `delete-material` at the same id, unconditional.

use super::mutation::CreateMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{delete_material, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateMaterial, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: payload.material.id.clone() })]
}
//#endregion 🔖️Inverse
