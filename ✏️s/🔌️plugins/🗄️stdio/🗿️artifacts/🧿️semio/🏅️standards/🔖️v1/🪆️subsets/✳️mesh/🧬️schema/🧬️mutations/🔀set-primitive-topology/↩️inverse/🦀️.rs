//! ↩️ Inverse for `SetPrimitiveTopology`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, primitive_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetPrimitiveTopology, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::SetPrimitiveTopology(super::SetPrimitiveTopology { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), topology: primitive.topology })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
