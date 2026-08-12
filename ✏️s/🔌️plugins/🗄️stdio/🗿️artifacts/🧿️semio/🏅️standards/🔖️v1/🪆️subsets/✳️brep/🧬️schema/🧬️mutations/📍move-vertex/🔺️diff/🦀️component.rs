//! 🔺️ `move-vertex` — sparse diff construction; an absent BASE `vertex_id` is a no-op clone.

use super::mutation::MoveVertex;
use crate::artifacts::semio::standards::v1::engine::triples::NamedModified;
use crate::artifacts::semio::standards::v1::engine::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepVertexDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveVertex, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if !base.vertices.iter().any(|v| v.id == payload.vertex_id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        vertices: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![NamedModified { key: payload.vertex_id.clone(), diff: BrepVertexDiff { point: Some(payload.new_point) } }],
            added: vec![],
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
