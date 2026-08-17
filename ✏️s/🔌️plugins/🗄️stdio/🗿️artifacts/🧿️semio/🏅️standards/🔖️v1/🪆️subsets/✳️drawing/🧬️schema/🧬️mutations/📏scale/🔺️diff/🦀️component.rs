//! 🔺️ `scale` — delegates to the shared `diff::diff_scale_node` helper; an absent `at` is
//! `mutation.target-missing` (Error, empty diff); a non-finite `new_scale` component is
//! `mutation.invariant` (Fatal, empty diff); a `new_scale` identical to a `Group` node's current
//! scale is `mutation.no-op` (Warning, empty diff). Every other node kind carries no scale field
//! of its own and stays an honest, unclassified no-op (see the payload leaf's own doc comment).

use super::mutation::Scale;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_scale_node, node_at, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &Scale, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
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
