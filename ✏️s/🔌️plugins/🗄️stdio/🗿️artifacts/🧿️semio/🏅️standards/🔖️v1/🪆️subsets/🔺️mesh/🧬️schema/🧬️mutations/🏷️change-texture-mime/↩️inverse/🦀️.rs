//! ↩️ Inverse for `ChangeTextureMime`.

use crate::standards::v1::subsets::mesh::schema::diff::texture_at;
use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeTextureMime, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
    match texture_at(base, &payload.id) {
        Some(texture) => vec![SemioMeshMutation::ChangeTextureMime(super::ChangeTextureMime { id: payload.id.clone(), new_mime: texture.mime.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
