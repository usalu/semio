//! ↩️ Inverse for `CreateMaterial`.

use crate::standards::v1::subsets::mesh::schema::mutations::{delete_material, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateMaterial, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeleteMaterial(delete_material::DeleteMaterial { id: payload.material.id.clone() })]
}
//#endregion 🔖️Inverse
