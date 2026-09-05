//! ↩️ Inverse for `DeleteTexture`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{SemioMeshMutation, create_texture};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteTexture, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    let Some(pos) = base.textures.iter().position(|t| t.id == payload.id) else {
        return Vec::new();
    };
    let tail = base.textures[pos + 1..].to_vec();
    let mut steps: Vec<SemioMeshMutation> = tail.iter().rev().map(|t| SemioMeshMutation::DeleteTexture(super::DeleteTexture { id: t.id.clone() })).collect();
    steps.push(SemioMeshMutation::CreateTexture(create_texture::CreateTexture { texture: base.textures[pos].clone() }));
    steps.extend(tail.into_iter().map(|t| SemioMeshMutation::CreateTexture(create_texture::CreateTexture { texture: t })));
    steps
}
//#endregion 🔖️Inverse
