//! ↩️ Inverse for `ChangeMaterialMetallic`.

use crate::standards::v1::subsets::mesh::schema::diff::material_at;
use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeMaterialMetallic, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match material_at(base, &payload.id) {
        Some(material) => vec![SemioMeshMutation::ChangeMaterialMetallic(super::ChangeMaterialMetallic { id: payload.id.clone(), new_metallic: material.metallic })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
