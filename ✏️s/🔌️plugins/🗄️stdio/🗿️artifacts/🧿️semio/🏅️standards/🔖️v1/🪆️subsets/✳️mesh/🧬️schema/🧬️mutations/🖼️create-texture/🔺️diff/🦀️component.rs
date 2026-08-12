//! 🔺️ `create-texture` — delegates to `schema::diff::diff_add_texture`, which no-ops on a
//! duplicate `id`.

use super::mutation::CreateTexture;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateTexture, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_texture(base, payload.texture.clone())
}
//#endregion 🔖️Diff
