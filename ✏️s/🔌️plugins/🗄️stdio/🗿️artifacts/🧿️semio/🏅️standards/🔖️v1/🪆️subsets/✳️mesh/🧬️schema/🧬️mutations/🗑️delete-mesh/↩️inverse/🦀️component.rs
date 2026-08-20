//! ↩️ `delete-mesh` — position-preserving reconstruction: `create-mesh` always APPENDS, so
//! naively reinverting to a single `create-mesh` would restore the mesh's VALUE but lose its
//! ORIGINAL POSITION whenever other meshes originally followed it. Restores exact position by
//! first deleting every mesh that originally followed `id` (in reverse), then re-creating `id` and
//! each of them back in original order. Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{create_mesh, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteMesh, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(pos) = base.meshes.iter().position(|m| m.id == payload.id) else {
        return Vec::new();
    };
    let tail = base.meshes[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|m| SemioMeshMutation::DeleteMesh(DeleteMesh { id: m.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreateMesh(create_mesh::mutation::CreateMesh { mesh: base.meshes[pos].clone() }));
    steps.extend(tail.into_iter().map(|m| SemioMeshMutation::CreateMesh(create_mesh::mutation::CreateMesh { mesh: m })));
    steps
}
//#endregion 🔖️Inverse
