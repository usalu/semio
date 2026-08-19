//! ↩️ `delete-texture` — position-preserving reconstruction (same technique as `delete-mesh`'s
//! inverse). Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteTexture;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{create_texture, SemioMeshMutation};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteTexture, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(pos) = base.textures.iter().position(|t| t.id == payload.id) else {
        return Vec::new();
    };
    let tail = base.textures[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|t| SemioMeshMutation::DeleteTexture(DeleteTexture { id: t.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreateTexture(create_texture::mutation::CreateTexture { texture: base.textures[pos].clone() }));
    steps.extend(tail.into_iter().map(|t| SemioMeshMutation::CreateTexture(create_texture::mutation::CreateTexture { texture: t })));
    steps
}
//#endregion 🔖️Inverse
