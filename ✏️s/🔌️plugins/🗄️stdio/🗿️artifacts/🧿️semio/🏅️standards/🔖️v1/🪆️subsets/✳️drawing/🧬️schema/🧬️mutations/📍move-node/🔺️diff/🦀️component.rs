//! 🔺️ `move-node` — delegates to the shared `diff::diff_move_node` helper (reused by
//! `drag-nodes`); an absent `at` is `mutation.target-missing` (Error, empty diff); a non-finite
//! `new_origin` component is `mutation.invariant` (Fatal, empty diff); a `new_origin` identical to
//! the node's current origin is `mutation.no-op` (Warning, empty diff). A `Path` node has no
//! origin field of its own and stays an honest, unclassified no-op (see `diff_move_node`'s own
//! doc comment).

use super::mutation::MoveNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_move_node, node_at, node_origin, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &MoveNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    if node_at(base, &payload.at).await.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]).await;
    }
    if !payload.new_origin.x.is_finite() || !payload.new_origin.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node at layer #{} new origin has a non-finite component.", payload.at.layer), [payload.at.layer.to_string()]).await;
    }
    if node_origin(base, &payload.at) == Some(payload.new_origin) {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Node in layer #{} is already at that position.", payload.at.layer)).await;
    }
    protocol::MutationOutcome::new(diff_move_node(base, &payload.at, payload.new_origin))
}
//#endregion 🔖️Diff
