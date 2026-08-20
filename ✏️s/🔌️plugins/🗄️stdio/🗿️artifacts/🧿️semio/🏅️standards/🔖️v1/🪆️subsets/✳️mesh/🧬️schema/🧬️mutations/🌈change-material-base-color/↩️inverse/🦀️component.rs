//! ↩️ `change-material-base-color` — undo sets `base_color` back to the BASE-state value; an
//! absent target ⇒ `Vec::new()`.

use super::mutation::ChangeMaterialBaseColor;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::material_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &ChangeMaterialBaseColor, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match material_at(base, &payload.id) {
        Some(material) => vec![SemioMeshMutation::ChangeMaterialBaseColor(ChangeMaterialBaseColor { id: payload.id.clone(), new_base_color: material.base_color })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
