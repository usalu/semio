//! 🔺️ Diff for `ReplacePrimitiveGeometry`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, primitive_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReplacePrimitiveGeometry, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(primitive) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id), [payload.primitive_id.clone()]);
    };
    if primitive.positions == payload.positions && primitive.normals == payload.normals && primitive.uvs == payload.uvs && primitive.colors == payload.colors && primitive.indices == payload.indices {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Primitive \"{}\" geometry is unchanged.", payload.primitive_id));
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_replace_primitive_geometry(
        base,
        &payload.mesh_id,
        &payload.primitive_id,
        payload.positions.clone(),
        payload.normals.clone(),
        payload.uvs.clone(),
        payload.colors.clone(),
        payload.indices.clone(),
    ))
}
//#endregion 🔖️Diff
