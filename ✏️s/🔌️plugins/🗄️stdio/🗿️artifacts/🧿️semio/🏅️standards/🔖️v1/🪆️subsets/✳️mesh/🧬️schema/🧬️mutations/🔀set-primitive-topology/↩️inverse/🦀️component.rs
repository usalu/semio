//! ↩️ `set-primitive-topology` — undo sets `topology` back to the BASE-state value; an absent
//! target ⇒ `Vec::new()`.

use super::mutation::SetPrimitiveTopology;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &SetPrimitiveTopology, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::SetPrimitiveTopology(SetPrimitiveTopology { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), topology: primitive.topology })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
