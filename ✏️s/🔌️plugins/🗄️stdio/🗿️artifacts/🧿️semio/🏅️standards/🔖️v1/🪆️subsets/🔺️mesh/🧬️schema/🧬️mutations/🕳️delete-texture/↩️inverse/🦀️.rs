//! ↩️ Restores the deleted texture at its original position without disturbing referenced siblings.

use crate::standards::v1::subsets::mesh::schema::mutations::{create_texture, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteTexture, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok(base
        .textures
        .iter()
        .position(|texture| texture.id == payload.id)
        .map(|at| SemioMeshMutation::CreateTexture(create_texture::CreateTexture { texture: base.textures[at].clone(), at: Some(at) }))
        .into_iter()
        .collect())
}
//#endregion 🔖️Inverse
