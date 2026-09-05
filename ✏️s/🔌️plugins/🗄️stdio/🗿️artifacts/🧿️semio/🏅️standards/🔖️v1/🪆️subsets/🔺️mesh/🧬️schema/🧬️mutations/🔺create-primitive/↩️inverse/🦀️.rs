//! ↩️ Inverse for `CreatePrimitive`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, delete_primitive};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioPrimitive};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreatePrimitive, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeletePrimitive(delete_primitive::DeletePrimitive { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive.id.clone() })]
}
//#endregion 🔖️Inverse
