//! ↩️ Inverse for `DeleteMaterial`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, create_material};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteMaterial, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(pos) = base.materials.iter().position(|m| m.id == payload.id) else {
        return Vec::new();
    };
    let tail = base.materials[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|m| SemioMeshMutation::DeleteMaterial(super::DeleteMaterial { id: m.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreateMaterial(create_material::CreateMaterial { material: base.materials[pos].clone() }));
    steps.extend(tail.into_iter().map(|m| SemioMeshMutation::CreateMaterial(create_material::CreateMaterial { material: m })));
    steps
}
//#endregion 🔖️Inverse
