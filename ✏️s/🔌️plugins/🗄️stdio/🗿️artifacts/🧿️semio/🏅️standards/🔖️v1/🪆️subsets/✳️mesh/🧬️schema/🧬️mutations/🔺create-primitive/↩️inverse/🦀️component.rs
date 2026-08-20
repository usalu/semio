//! ↩️ `create-primitive` — undo is `delete-primitive` at the same (`mesh_id`, `primitive_id`),
//! unconditional (mirrors `create-mesh`'s inverse).

use super::mutation::CreatePrimitive;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{delete_primitive, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreatePrimitive, _base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    vec![SemioMeshMutation::DeletePrimitive(delete_primitive::mutation::DeletePrimitive { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive.id.clone() })]
}
//#endregion 🔖️Inverse
