//! ↩️ `delete-primitive` — position-preserving reconstruction scoped to `mesh_id`'s own
//! `primitives` collection (same technique as `delete-mesh`'s inverse). Missing mesh or primitive
//! ⇒ `Vec::new()`.

use super::mutation::DeletePrimitive;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::mesh_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{create_primitive, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeletePrimitive, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(mesh) = mesh_at(base, &payload.mesh_id) else {
        return Vec::new();
    };
    let Some(pos) = mesh.primitives.iter().position(|p| p.id == payload.primitive_id) else {
        return Vec::new();
    };
    let tail = mesh.primitives[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|p| SemioMeshMutation::DeletePrimitive(DeletePrimitive { mesh_id: payload.mesh_id.clone(), primitive_id: p.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreatePrimitive(create_primitive::mutation::CreatePrimitive { mesh_id: payload.mesh_id.clone(), primitive: mesh.primitives[pos].clone() }));
    steps.extend(tail.into_iter().map(|p| SemioMeshMutation::CreatePrimitive(create_primitive::mutation::CreatePrimitive { mesh_id: payload.mesh_id.clone(), primitive: p })));
    steps
}
//#endregion 🔖️Inverse
