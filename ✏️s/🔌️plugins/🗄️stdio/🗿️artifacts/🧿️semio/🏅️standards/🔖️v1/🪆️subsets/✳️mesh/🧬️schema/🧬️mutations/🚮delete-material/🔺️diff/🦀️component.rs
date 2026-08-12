//! 🔺️ `delete-material` — delegates to `schema::diff::diff_remove_material`, which returns a
//! genuinely empty diff when `id` is absent.

use super::mutation::DeleteMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_material(base, &payload.id)
}
//#endregion 🔖️Diff
