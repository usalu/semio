//! ↩️ Inverse for `CreateTexture`.

use crate::standards::v1::subsets::mesh::schema::mutations::{delete_texture, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateTexture, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.textures.iter().any(|texture| texture.id == payload.texture.id) {
        return Vec::new();
    }
    vec![SemioMeshMutation::DeleteTexture(delete_texture::DeleteTexture { id: payload.texture.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
