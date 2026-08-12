//! 🔺️ `change-material-base-color` — delegates to `schema::diff::diff_change_material_base_color`,
//! which returns a genuinely empty diff when `id` is absent.

use super::mutation::ChangeMaterialBaseColor;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMaterialBaseColor, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_change_material_base_color(base, &payload.id, payload.new_base_color)
}
//#endregion 🔖️Diff
