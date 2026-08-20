//! ✂️ `delete-primitive` — Error `mutation.target-missing` when the (`mesh_id`,`primitive_id`)
//! pair is absent.

use super::mutation::DeletePrimitive;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &DeletePrimitive, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    if crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id), [payload.primitive_id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_remove_primitive(base, &payload.mesh_id, &payload.primitive_id))
}
//#endregion 🔖️Diff
