//! 🔺️ `change-texture-mime` — delegates to `schema::diff::diff_change_texture_mime`, which
//! returns a genuinely empty diff when `id` is absent.

use super::mutation::ChangeTextureMime;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTextureMime, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_texture_mime(base, &payload.id, payload.new_mime.clone())
}
//#endregion 🔖️Diff
