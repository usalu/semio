//! 🔺️ `set-primitive-topology` — delegates to `schema::diff::diff_set_primitive_topology`, which
//! returns a genuinely empty diff when the (`mesh_id`,`primitive_id`) pair is absent.

use super::mutation::SetPrimitiveTopology;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &SetPrimitiveTopology, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_primitive_topology(base, &payload.mesh_id, &payload.primitive_id, payload.topology)
}
//#endregion 🔖️Diff
