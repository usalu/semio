//! 🔺️ `drag-nodes` — folds one `diff_move_node` per addressed node into a single accumulated
//! diff via `MutationDiff::absorb` (each node's own current origin read from `base`, offset
//! applied independently -- never apply-then-capture).

use super::mutation::DragNodes;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_move_node, node_origin, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::MutationDiff;

//#region 🔖️Diff
pub fn diff(payload: &DragNodes, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    let mut acc = SemioDrawingDiff::default();
    for at in &payload.ats {
        if let Some(origin) = node_origin(base, at) {
            let new_origin = crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2 { x: origin.x + payload.offset.x, y: origin.y + payload.offset.y };
            acc.absorb(diff_move_node(base, at, new_origin));
        }
    }
    acc
}
//#endregion 🔖️Diff
