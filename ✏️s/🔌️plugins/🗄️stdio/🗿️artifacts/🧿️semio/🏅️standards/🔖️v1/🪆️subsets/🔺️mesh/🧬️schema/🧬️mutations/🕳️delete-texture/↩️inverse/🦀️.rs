//! ↩️ Restores the deleted texture at its original position without disturbing referenced siblings.

use crate::schema::mutations::create_texture;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeleteTexture, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_texture::CreateTexture>,
{
    Ok(base
        .textures
        .iter()
        .position(|texture| texture.id == payload.id)
        .map(|at| Op::from(create_texture::CreateTexture { texture: base.textures[at].clone(), at: Some(at) }))
        .into_iter()
        .collect())
}
//#endregion 🔖️Inverse
