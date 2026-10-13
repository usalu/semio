//! ↩️ Inverse for `ReplaceTextureBytes`.

use crate::schema::diff::texture_at;
use crate::schema::mutations::SemioMeshMutation;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReplaceTextureBytes, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match texture_at(base, &payload.id) {
        Some(texture) => vec![SemioMeshMutation::ReplaceTextureBytes(super::ReplaceTextureBytes { id: payload.id.clone(), new_bytes: texture.bytes.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
