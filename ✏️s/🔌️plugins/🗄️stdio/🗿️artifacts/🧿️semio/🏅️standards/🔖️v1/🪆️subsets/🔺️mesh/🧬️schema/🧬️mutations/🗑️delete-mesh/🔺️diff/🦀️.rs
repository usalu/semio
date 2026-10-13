//! 🔺️ Diff for `DeleteMesh`.

use crate::schema::diff::SemioMeshDiff;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteMesh, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::schema::diff::mesh_at(base, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mesh \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::schema::diff::diff_remove_mesh(base, &payload.id))
}
//#endregion 🔖️Diff
