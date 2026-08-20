//! ↩️ `create-material` — undo is `delete-material` at the same id, unconditional.

use super::mutation::CreateMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{delete_material, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreateMaterial, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: payload.material.id.clone() })]
}
//#endregion 🔖️Inverse
