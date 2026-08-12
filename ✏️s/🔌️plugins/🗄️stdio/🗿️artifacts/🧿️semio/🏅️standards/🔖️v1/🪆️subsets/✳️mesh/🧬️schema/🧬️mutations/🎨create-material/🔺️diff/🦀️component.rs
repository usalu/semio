//! 🔺️ `create-material` — delegates to `schema::diff::diff_add_material`, which no-ops on a
//! duplicate `id`.

use super::mutation::CreateMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_material(base, payload.material.clone())
}
//#endregion 🔖️Diff
