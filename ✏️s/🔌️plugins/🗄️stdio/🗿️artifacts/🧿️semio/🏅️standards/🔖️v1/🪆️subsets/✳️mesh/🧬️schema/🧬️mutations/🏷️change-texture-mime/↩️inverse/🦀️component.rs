//! ↩️ `change-texture-mime` — undo sets `mime` back to the BASE-state value; an absent target ⇒
//! `Vec::new()`.

use super::mutation::ChangeTextureMime;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::texture_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeTextureMime, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match texture_at(base, &payload.id).await {
        Some(texture) => vec![SemioMeshMutation::ChangeTextureMime(ChangeTextureMime { id: payload.id.clone(), new_mime: texture.mime.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
