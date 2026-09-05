//! 🔺️ Diff for `CreatePrimitive`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioPrimitive};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreatePrimitive, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(mesh) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::mesh_at(base, &payload.mesh_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mesh \"{}\" does not exist.", payload.mesh_id), [payload.mesh_id.clone()]);
    };
    if mesh.primitives.iter().any(|p| p.id == payload.primitive.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Primitive \"{}\" already exists in mesh \"{}\".", payload.primitive.id, payload.mesh_id), [payload.primitive.id.clone()]);
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_add_primitive(base, &payload.mesh_id, payload.primitive.clone()))
}
//#endregion 🔖️Diff
