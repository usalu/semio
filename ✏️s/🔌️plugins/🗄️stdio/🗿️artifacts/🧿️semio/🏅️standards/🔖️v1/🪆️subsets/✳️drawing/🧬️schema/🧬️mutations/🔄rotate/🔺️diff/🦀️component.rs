//! 🔺️ `rotate` — delegates to the shared `diff::diff_rotate_node` helper; an absent `at` is
//! `mutation.target-missing` (Error, empty diff); a non-finite `new_rotation` component is
//! `mutation.invariant` (Fatal, empty diff); a `new_rotation` identical to a `Group` node's
//! current rotation is `mutation.no-op` (Warning, empty diff). `Path`/`Text`/`Image` carry no
//! rotation field of their own and stay an honest, unclassified no-op (see the payload leaf's own
//! doc comment).

use super::mutation::Rotate;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_rotate_node, node_at, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &Rotate, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(node) = node_at(base, &payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]);
    };
    let r = payload.new_rotation;
    if !r.x.is_finite() || !r.y.is_finite() || !r.z.is_finite() || !r.w.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node at layer #{} new rotation has a non-finite component.", payload.at.layer), [payload.at.layer.to_string()]);
    }
    if let DrawNode::Group { transform, .. } = node {
        if transform.rotation == r {
            return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Node in layer #{} already has that rotation.", payload.at.layer));
        }
    }
    protocol::MutationOutcome::new(diff_rotate_node(base, &payload.at, r))
}
//#endregion 🔖️Diff
