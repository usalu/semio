//! 🔺️ Diff for `ScaleNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_scale_node, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ScaleNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(node) = node_at(base, &payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]);
    };
    let s = payload.new_scale;
    if !s.x.is_finite() || !s.y.is_finite() || !s.z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node at layer #{} new scale has a non-finite component.", payload.at.layer), [payload.at.layer.to_string()]);
    }
    if let DrawNode::Group { transform, .. } = node {
        if transform.scale == s {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} already has that scale.", payload.at.layer));
        }
    }
    protocol::MutationOutcome::new(diff_scale_node(base, &payload.at, s))
}
//#endregion 🔖️Diff
