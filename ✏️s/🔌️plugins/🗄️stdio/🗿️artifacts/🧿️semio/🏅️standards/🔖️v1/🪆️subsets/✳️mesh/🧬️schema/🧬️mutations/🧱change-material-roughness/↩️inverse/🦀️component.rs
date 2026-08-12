//! ↩️ `change-material-roughness` — undo sets `roughness` back to the BASE-state value; an absent
//! target ⇒ `Vec::new()`.

use super::mutation::ChangeMaterialRoughness;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeMaterialRoughness, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match material_at(base, &payload.id) {
        Some(material) => vec![SemioMeshMutation::ChangeMaterialRoughness(ChangeMaterialRoughness { id: payload.id.clone(), new_roughness: material.roughness })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
