//! ↩️ `replace-primitive-geometry` — undo restores the FULL BASE-state buffer set; an absent
//! target ⇒ `Vec::new()`.

use super::mutation::ReplacePrimitiveGeometry;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &ReplacePrimitiveGeometry, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::ReplacePrimitiveGeometry(ReplacePrimitiveGeometry {
            mesh_id: payload.mesh_id.clone(),
            primitive_id: payload.primitive_id.clone(),
            positions: primitive.positions.clone(),
            normals: primitive.normals.clone(),
            uvs: primitive.uvs.clone(),
            colors: primitive.colors.clone(),
            indices: primitive.indices.clone(),
        })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
