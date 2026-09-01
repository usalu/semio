//! ↩️ Inverse for `DeleteMesh`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, create_mesh};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteMesh, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(pos) = base.meshes.iter().position(|m| m.id == payload.id) else {
        return Vec::new();
    };
    let tail = base.meshes[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|m| SemioMeshMutation::DeleteMesh(super::DeleteMesh { id: m.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: base.meshes[pos].clone() }));
    steps.extend(tail.into_iter().map(|m| SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: m })));
    steps
}
//#endregion 🔖️Inverse
