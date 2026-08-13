//! 🔺️ `group` — sparse diff construction; a no-op when `parent` is not a `Group`, `indices` is
//! empty, or `indices` is not a contiguous ascending run of valid children indices (the shape
//! `ungroup`'s own inverse relies on to restore the exact original membership/positions).

use super::mutation::GroupNodes;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️ContiguousCheck
/// ✅️️ `true` iff `indices` is non-empty, strictly ascending, and every consecutive pair differs by
/// exactly 1 (a contiguous run) -- shared by this triad's `diff` and `↩️inverse/🦀️component.rs`'s
/// own reverse construction (via `ungroup`, which always emits a genuinely contiguous run).
pub(crate) fn is_contiguous_ascending(indices: &[usize]) -> bool {
    !indices.is_empty() && indices.windows(2).all(|w| w[1] == w[0] + 1)
}
//#endregion 🔖️ContiguousCheck

//#region 🔖️Diff
pub fn diff(payload: &GroupNodes, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    if !is_contiguous_ascending(&payload.indices) {
        return SemioDrawingDiff::default();
    }
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.indices.iter().all(|&i| i < children.len()) => {
            let grouped: Vec<DrawNode> = payload.indices.iter().map(|&i| children[i].clone()).collect();
            let new_group = DrawNode::Group { transform: payload.transform, children: grouped };
            let at = payload.indices[0];
            diff_at_path(&payload.parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: payload.indices.clone(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: new_group }] }) }))
        }
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
