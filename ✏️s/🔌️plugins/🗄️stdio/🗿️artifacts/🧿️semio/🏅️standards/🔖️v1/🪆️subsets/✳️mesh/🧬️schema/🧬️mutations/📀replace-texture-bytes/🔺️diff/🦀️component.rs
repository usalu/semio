//! 🔺️ `replace-texture-bytes` — delegates to `schema::diff::diff_replace_texture_bytes`, which
//! returns a genuinely empty diff when `id` is absent.

use super::mutation::ReplaceTextureBytes;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceTextureBytes, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_replace_texture_bytes(base, &payload.id, payload.new_bytes.clone())
}
//#endregion 🔖️Diff
