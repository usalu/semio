//! 🔺️ Diff for `ReplaceTextureBytes`.

use crate::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReplaceTextureBytes, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(texture) = crate::standards::v1::subsets::mesh::schema::diff::texture_at(base, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Texture \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if texture.bytes == payload.new_bytes {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Texture \"{}\" bytes are unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(crate::standards::v1::subsets::mesh::schema::diff::diff_replace_texture_bytes(base, &payload.id, payload.new_bytes.clone()))
}
//#endregion 🔖️Diff
