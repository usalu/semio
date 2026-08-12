//! 🔺️ `create-primitive` — delegates to `schema::diff::diff_add_primitive`, which no-ops when
//! `mesh_id` is absent or `primitive.id` already exists in that mesh.

use super::mutation::CreatePrimitive;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreatePrimitive, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_primitive(base, &payload.mesh_id, payload.primitive.clone())
}
//#endregion 🔖️Diff
