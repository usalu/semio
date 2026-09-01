//! 🔺️ Diff for `SetPrimitiveMaterial`.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, primitive_at};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetPrimitiveMaterial, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(primitive) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id), [payload.primitive_id.clone()]);
    };
    if primitive.material_id == payload.material_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Primitive \"{}\" material is unchanged.", payload.primitive_id));
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_set_primitive_material(base, &payload.mesh_id, &payload.primitive_id, payload.material_id.clone()))
}
//#endregion 🔖️Diff
