//! 🔺️ `change-material-metallic` — delegates to `schema::diff::diff_change_material_metallic`,
//! which returns a genuinely empty diff when `id` is absent.

use super::mutation::ChangeMaterialMetallic;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMaterialMetallic, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_metallic(base, &payload.id, payload.new_metallic)
}
//#endregion 🔖️Diff
