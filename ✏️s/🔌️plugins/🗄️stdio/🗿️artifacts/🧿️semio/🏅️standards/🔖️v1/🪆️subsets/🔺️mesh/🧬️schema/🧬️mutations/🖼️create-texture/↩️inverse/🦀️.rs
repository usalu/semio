//! ↩️ Inverse for `CreateTexture`.

use crate::schema::mutations::delete_texture;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateTexture, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_texture::DeleteTexture>,
{
    Ok((|| {
    if base.textures.iter().any(|texture| texture.id == payload.texture.id) {
        return Vec::new();
    }
    vec![Op::from(delete_texture::DeleteTexture { id: payload.texture.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
