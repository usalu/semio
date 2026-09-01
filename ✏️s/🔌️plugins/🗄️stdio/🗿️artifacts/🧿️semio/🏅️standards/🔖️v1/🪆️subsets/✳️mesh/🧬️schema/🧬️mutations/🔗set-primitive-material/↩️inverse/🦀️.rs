//! ↩️ Inverse for `SetPrimitiveMaterial`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, primitive_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetPrimitiveMaterial, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::SetPrimitiveMaterial(super::SetPrimitiveMaterial { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), material_id: primitive.material_id.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
