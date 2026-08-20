//! ↩️ `move-vertex` — undo sets the same index back to the BASE-state point; an absent
//! (`mesh_id`,`primitive_id`) or an out-of-bounds `vertex_index` ⇒ `Vec::new()`.

use super::mutation::MoveVertex;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &MoveVertex, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(primitive) = primitive_at(base, &payload.mesh_id, &payload.primitive_id).await else {
        return Vec::new();
    };
    let Some(old_point) = primitive.positions.get(payload.vertex_index).copied() else {
        return Vec::new();
    };
    vec![SemioMeshMutation::MoveVertex(MoveVertex { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), vertex_index: payload.vertex_index, new_point: old_point })]
}
//#endregion 🔖️Inverse
