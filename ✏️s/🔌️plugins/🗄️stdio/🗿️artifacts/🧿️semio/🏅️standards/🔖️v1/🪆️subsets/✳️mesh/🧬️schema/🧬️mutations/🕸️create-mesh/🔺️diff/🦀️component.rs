//! 🔺️ `create-mesh` — delegates to `schema::diff::diff_add_mesh`, which itself no-ops (returns
//! `SemioMeshDiff::default()`) on a duplicate `id`.

use super::mutation::CreateMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMesh, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_mesh(base, payload.mesh.clone())
}
//#endregion 🔖️Diff
