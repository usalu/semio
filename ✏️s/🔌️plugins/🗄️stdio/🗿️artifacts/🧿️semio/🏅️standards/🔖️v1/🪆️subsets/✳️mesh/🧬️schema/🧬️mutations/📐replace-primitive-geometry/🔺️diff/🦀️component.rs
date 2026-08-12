//! 🔺️ `replace-primitive-geometry` — delegates to `schema::diff::diff_replace_primitive_geometry`,
//! which returns a genuinely empty diff when the (`mesh_id`,`primitive_id`) pair is absent.

use super::mutation::ReplacePrimitiveGeometry;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplacePrimitiveGeometry, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_replace_primitive_geometry(
        base,
        &payload.mesh_id,
        &payload.primitive_id,
        payload.positions.clone(),
        payload.normals.clone(),
        payload.uvs.clone(),
        payload.colors.clone(),
        payload.indices.clone(),
    )
}
//#endregion 🔖️Diff
