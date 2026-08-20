//! 🔺️ `move-vertex` — sparse diff construction; an absent BASE `vertex_id` is
//! `mutation.target-missing` (Error, empty diff); a `new_point` identical to the vertex's
//! current point is `mutation.no-op` (Warning, empty diff); a non-finite `new_point` component
//! is `mutation.invariant` (Fatal, empty diff).

use super::mutation::MoveVertex;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedModified;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepVertexDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &MoveVertex, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    let Some(vertex) = base.vertices.iter().find(|v| v.id == payload.vertex_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Vertex \"{}\" does not exist.", payload.vertex_id), [payload.vertex_id.clone()]);
    };
    let p = payload.new_point;
    if !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Vertex \"{}\" new point has a non-finite component.", payload.vertex_id), [payload.vertex_id.clone()]);
    }
    if vertex.point == p {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Vertex \"{}\" is already at this point.", payload.vertex_id));
    }
    protocol::MutationOutcome::new(SemioBrepDiff { vertices: Some(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: payload.vertex_id.clone(), diff: BrepVertexDiff { point: Some(p) } }], added: vec![] }), ..Default::default() })
}
//#endregion 🔖️Diff
