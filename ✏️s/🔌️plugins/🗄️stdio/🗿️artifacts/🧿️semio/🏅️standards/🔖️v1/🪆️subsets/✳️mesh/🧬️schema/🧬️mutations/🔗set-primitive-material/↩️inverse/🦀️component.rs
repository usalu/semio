//! ↩️ `set-primitive-material` — undo sets `material_id` back to the BASE-state value; an absent
//! target ⇒ `Vec::new()`.

use super::mutation::SetPrimitiveMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &SetPrimitiveMaterial, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::SetPrimitiveMaterial(SetPrimitiveMaterial { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), material_id: primitive.material_id.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
