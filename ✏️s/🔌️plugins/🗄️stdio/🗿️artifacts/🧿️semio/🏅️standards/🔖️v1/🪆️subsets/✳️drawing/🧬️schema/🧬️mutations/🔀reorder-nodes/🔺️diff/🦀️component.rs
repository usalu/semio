//! 🔺️ `reorder-nodes` — sparse diff construction; `parent` not resolving to a `Group` or `from`
//! being out of range for its `children` is `mutation.target-missing` (Error, empty diff); `from`
//! equal to `to` (already in that order) is `mutation.no-op` (Warning, empty diff).

use super::mutation::ReorderNodes;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ReorderNodes, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.from < children.len() => {
            if payload.from == payload.to {
                return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node #{} in layer #{} is already at position #{}.", payload.from, payload.parent.layer, payload.to));
            }
            let item = children[payload.from].clone();
            protocol::MutationOutcome::new(diff_at_path(&payload.parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![payload.from], modified: Vec::new(), added: vec![IndexAdded { index: payload.to, item }] }) })))
        }
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Parent at layer #{} does not resolve to a group, or index #{} is out of range.", payload.parent.layer, payload.from), [payload.parent.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
