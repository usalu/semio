//! 🔺️ `group` — sparse diff construction; `indices` being empty or not a contiguous ascending run
//! (the shape `ungroup`'s own inverse relies on to restore the exact original membership/
//! positions), or `parent` not resolving to a `Group` with every index in range, is
//! `mutation.target-missing` (Error, empty diff).

use super::mutation::GroupNodes;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️ContiguousCheck
/// ✅️️ `true` iff `indices` is non-empty, strictly ascending, and every consecutive pair differs by
/// exactly 1 (a contiguous run) -- shared by this triad's `diff` and `↩️inverse/🦀️component.rs`'s
/// own reverse construction (via `ungroup`, which always emits a genuinely contiguous run).
pub(crate) async fn is_contiguous_ascending(indices: &[usize]) -> bool {
    !indices.is_empty() && indices.windows(2).all(|w| w[1] == w[0] + 1)
}
//#endregion 🔖️ContiguousCheck

//#region 🔖️Diff
pub async fn diff(payload: &GroupNodes, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    if !is_contiguous_ascending(&payload.indices) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Indices for group in layer #{} are empty or not a contiguous ascending run.", payload.parent.layer), [payload.parent.layer.to_string()]).await;
    }
    match node_at(base, &payload.parent).await {
        Some(DrawNode::Group { children, .. }) if payload.indices.iter().all(|&i| i < children.len()) => {
            let grouped: Vec<DrawNode> = payload.indices.iter().map(|&i| children[i].clone()).collect();
            let new_group = DrawNode::Group { transform: payload.transform, children: grouped };
            let at = payload.indices[0];
            protocol::MutationOutcome::new(diff_at_path(&payload.parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: payload.indices.clone(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: new_group }] }) })))
        }
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Parent at layer #{} does not resolve to a group, or an index is out of range.", payload.parent.layer), [payload.parent.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
