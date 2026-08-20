//! ↩️ `replace-texture-bytes` — undo restores the FULL BASE-state byte payload; an absent target
//! ⇒ `Vec::new()`.

use super::mutation::ReplaceTextureBytes;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::texture_at;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplaceTextureBytes, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match texture_at(base, &payload.id).await {
        Some(texture) => vec![SemioMeshMutation::ReplaceTextureBytes(ReplaceTextureBytes { id: payload.id.clone(), new_bytes: texture.bytes.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
