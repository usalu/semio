//! 🔺️ `set-primitive-material` — delegates to `schema::diff::diff_set_primitive_material`, which
//! returns a genuinely empty diff when the (`mesh_id`,`primitive_id`) pair is absent.

use super::mutation::SetPrimitiveMaterial;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &SetPrimitiveMaterial, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_primitive_material(base, &payload.mesh_id, &payload.primitive_id, payload.material_id.clone())
}
//#endregion 🔖️Diff
