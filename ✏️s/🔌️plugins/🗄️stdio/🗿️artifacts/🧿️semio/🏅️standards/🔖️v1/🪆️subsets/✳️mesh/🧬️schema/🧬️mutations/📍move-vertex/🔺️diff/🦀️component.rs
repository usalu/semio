//! 📍 `move-vertex` — Error `mutation.target-missing` when the (`mesh_id`,`primitive_id`) pair is
//! absent OR `vertex_index` is out of bounds for that primitive's `positions`, Warning
//! `mutation.no-op` when `new_point` already equals the current position, Fatal
//! `mutation.invariant` when `new_point` has a non-finite coordinate.

use super::mutation::MoveVertex;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &MoveVertex, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<SemioMeshDiff> {
    let Some(primitive) = crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::primitive_at(base, &payload.mesh_id, &payload.primitive_id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Primitive \"{}\" does not exist in mesh \"{}\".", payload.primitive_id, payload.mesh_id),
            [format!("{}:{}:{}", payload.mesh_id, payload.primitive_id, payload.vertex_index)],
        );
    };
    let Some(current) = primitive.positions.get(payload.vertex_index) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Vertex {} does not exist on primitive \"{}\".", payload.vertex_index, payload.primitive_id),
            [format!("{}:{}:{}", payload.mesh_id, payload.primitive_id, payload.vertex_index)],
        );
    };
    if *current == payload.new_point {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Vertex {} of primitive \"{}\" is already at that position.", payload.vertex_index, payload.primitive_id));
    }
    if !payload.new_point.x.is_finite() || !payload.new_point.y.is_finite() || !payload.new_point.z.is_finite() {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Vertex {} target position for primitive \"{}\" is not finite.", payload.vertex_index, payload.primitive_id),
            [format!("{}:{}:{}", payload.mesh_id, payload.primitive_id, payload.vertex_index)],
        );
    }
    protocol::MutationOutcome::new(crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::diff_move_vertex(base, &payload.mesh_id, &payload.primitive_id, payload.vertex_index, payload.new_point))
}
//#endregion 🔖️Diff
