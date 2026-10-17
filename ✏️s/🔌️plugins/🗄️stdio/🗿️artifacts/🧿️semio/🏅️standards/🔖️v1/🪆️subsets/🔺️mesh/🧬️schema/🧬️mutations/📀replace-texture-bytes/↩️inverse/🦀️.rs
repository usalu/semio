//! ↩️ Inverse for `ReplaceTextureBytes`.

use crate::schema::diff::texture_at;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ReplaceTextureBytes, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::replace_texture_bytes::ReplaceTextureBytes>,
{
    Ok((|| {
    match texture_at(base, &payload.id) {
        Some(texture) => vec![Op::from(super::ReplaceTextureBytes { id: payload.id.clone(), new_bytes: texture.bytes.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
