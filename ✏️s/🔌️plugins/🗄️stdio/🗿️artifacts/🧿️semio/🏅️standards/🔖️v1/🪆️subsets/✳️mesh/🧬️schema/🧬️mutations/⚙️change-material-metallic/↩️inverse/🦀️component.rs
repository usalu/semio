//! ↩️ `change-material-metallic` — undo sets `metallic` back to the BASE-state value; an absent
//! target ⇒ `Vec::new()`.

use super::mutation::ChangeMaterialMetallic;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeMaterialMetallic, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match material_at(base, &payload.id).await {
        Some(material) => vec![SemioMeshMutation::ChangeMaterialMetallic(ChangeMaterialMetallic { id: payload.id.clone(), new_metallic: material.metallic })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
