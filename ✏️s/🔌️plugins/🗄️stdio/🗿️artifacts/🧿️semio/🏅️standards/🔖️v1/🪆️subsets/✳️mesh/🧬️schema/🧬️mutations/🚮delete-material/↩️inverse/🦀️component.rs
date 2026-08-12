//! ↩️ `delete-material` — position-preserving reconstruction (same technique as `delete-mesh`'s
//! inverse). Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{create_material, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteMaterial, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(pos) = base.materials.iter().position(|m| m.id == payload.id) else {
        return Vec::new();
    };
    let tail = base.materials[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|m| SemioMeshMutation::DeleteMaterial(DeleteMaterial { id: m.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: base.materials[pos].clone() }));
    steps.extend(tail.into_iter().map(|m| SemioMeshMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: m })));
    steps
}
//#endregion 🔖️Inverse
