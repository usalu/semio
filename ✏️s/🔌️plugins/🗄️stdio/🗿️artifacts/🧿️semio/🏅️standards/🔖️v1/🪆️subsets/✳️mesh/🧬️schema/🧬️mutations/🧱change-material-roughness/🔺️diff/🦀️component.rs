//! 🔺️ `change-material-roughness` — delegates to `schema::diff::diff_change_material_roughness`,
//! which returns a genuinely empty diff when `id` is absent.

use super::mutation::ChangeMaterialRoughness;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMaterialRoughness, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_roughness(base, &payload.id, payload.new_roughness)
}
//#endregion 🔖️Diff
