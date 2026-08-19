//! 🔺️ `ungroup` — sparse diff construction; `at` being the layer root (empty `path`, no parent to
//! splice into) or not resolving to a `Group` is `mutation.target-missing` (Error, empty diff).

use super::mutation::UngroupNode;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::delete_node::diff::parent_and_index;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &UngroupNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some((parent, group_index)) = parent_and_index(&payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} has no parent to ungroup into (layer root).", payload.at.layer), [payload.at.layer.to_string()]);
    };
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { children, .. }) => {
            let added: Vec<IndexAdded<DrawNode>> = children.iter().enumerate().map(|(i, child)| IndexAdded { index: group_index + i, item: child.clone() }).collect();
            protocol::MutationOutcome::new(diff_at_path(&parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![group_index], modified: Vec::new(), added }) })))
        }
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist or is not a group.", payload.at.layer), [payload.at.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
