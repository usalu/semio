//! ↩️ Inverse for `DeletePrimitive`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, mesh_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, create_primitive};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeletePrimitive, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(mesh) = mesh_at(base, &payload.mesh_id) else {
        return Vec::new();
    };
    let Some(pos) = mesh.primitives.iter().position(|p| p.id == payload.primitive_id) else {
        return Vec::new();
    };
    let tail = mesh.primitives[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|p| SemioMeshMutation::DeletePrimitive(super::DeletePrimitive { mesh_id: payload.mesh_id.clone(), primitive_id: p.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreatePrimitive(create_primitive::CreatePrimitive { mesh_id: payload.mesh_id.clone(), primitive: mesh.primitives[pos].clone() }));
    steps.extend(tail.into_iter().map(|p| SemioMeshMutation::CreatePrimitive(create_primitive::CreatePrimitive { mesh_id: payload.mesh_id.clone(), primitive: p })));
    steps
}
//#endregion 🔖️Inverse
