//! 🔺️ `delete-mesh` — delegates to `schema::diff::diff_remove_mesh`, which returns a genuinely
//! empty `SemioMeshDiff::default()` when `id` is absent from `base` (never a spurious `removed`
//! entry — the exact bug class brep's law-testing wave caught for its own `delete-*` triads).

use super::mutation::DeleteMesh;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMesh, base: &SemioMeshSnapshot) -> SemioMeshDiff {
    crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_mesh(base, &payload.id)
}
//#endregion 🔖️Diff
