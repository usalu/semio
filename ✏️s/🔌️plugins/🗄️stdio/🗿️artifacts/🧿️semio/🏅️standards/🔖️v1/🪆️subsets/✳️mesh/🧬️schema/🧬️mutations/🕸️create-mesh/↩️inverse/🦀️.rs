//! ↩️ Inverse for `CreateMesh`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, delete_mesh};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateMesh, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: payload.mesh.id.clone() })]
}
//#endregion 🔖️Inverse
