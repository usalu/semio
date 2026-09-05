//! 🔺️ Diff for `MoveNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_move_node, node_at, node_origin};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::MoveNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    if node_at(base, &payload.at).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]);
    }
    if !payload.new_origin.x.is_finite() || !payload.new_origin.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node at layer #{} new origin has a non-finite component.", payload.at.layer), [payload.at.layer.to_string()]);
    }
    if node_origin(base, &payload.at) == Some(payload.new_origin) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} is already at that position.", payload.at.layer));
    }
    protocol::MutationOutcome::new(diff_move_node(base, &payload.at, payload.new_origin))
}
//#endregion 🔖️Diff
