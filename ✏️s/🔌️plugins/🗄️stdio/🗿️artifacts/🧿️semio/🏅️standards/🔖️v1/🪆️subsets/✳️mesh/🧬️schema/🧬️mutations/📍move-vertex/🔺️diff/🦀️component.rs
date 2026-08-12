//! 🔺️ `move-vertex` — delegates to `schema::diff::diff_move_vertex`, which returns a genuinely
//! empty diff when the (`mesh_id`,`primitive_id`) pair is absent OR `vertex_index` is out of
//! bounds for that primitive's `positions`.

use super::mutation::MoveVertex;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveVertex, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_move_vertex(base, &payload.mesh_id, &payload.primitive_id, payload.vertex_index, payload.new_point)
}
//#endregion 🔖️Diff
