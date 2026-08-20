//! 🔺️ `create-node` — sparse diff construction; `parent` not resolving to a `Group` (unknown
//! container reference — `DrawNode` has no stable id, so there is no separate `✅validation-report`
//! facet in this subset to defer to) is `mutation.invariant` (Fatal, empty diff).

use super::mutation::CreateNode;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match node_at(base, &payload.parent).await {
        Some(DrawNode::Group { children, .. }) => {
            let at = payload.index.min(children.len());
            protocol::MutationOutcome::new(diff_at_path(&payload.parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: payload.node.clone() }] }) })))
        }
        _ => protocol::MutationOutcome::fatal("mutation.invariant", format!("Parent at layer #{} does not resolve to a group.", payload.parent.layer), [payload.parent.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
