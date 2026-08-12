//! 🔺️ `delete-texture` — delegates to `schema::diff::diff_remove_texture`, which returns a
//! genuinely empty diff when `id` is absent.

use super::mutation::DeleteTexture;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteTexture, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_texture(base, &payload.id)
}
//#endregion 🔖️Diff
