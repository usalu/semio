//! 🔺️ `delete-primitive` — delegates to `schema::diff::diff_remove_primitive`, which returns a
//! genuinely empty `SemioMeshDiff::default()` when the (`mesh_id`,`primitive_id`) pair is absent.

use super::mutation::DeletePrimitive;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeletePrimitive, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_primitive(base, &payload.mesh_id, &payload.primitive_id)
}
//#endregion 🔖️Diff
