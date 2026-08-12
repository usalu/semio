//! 🔺️ `ungroup` — sparse diff construction; a no-op when `at` does not resolve to a `Group` or is
//! the layer root (empty `path`, no parent to splice into).

use super::mutation::UngroupNode;
use crate::artifacts::semio::standards::v1::engine::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::delete_node::diff::parent_and_index;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &UngroupNode, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    let Some((parent, group_index)) = parent_and_index(&payload.at) else { return SemioDrawingDiff::default() };
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { children, .. }) => {
            let added: Vec<IndexAdded<DrawNode>> = children.iter().enumerate().map(|(i, child)| IndexAdded { index: group_index + i, item: child.clone() }).collect();
            diff_at_path(&parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![group_index], modified: Vec::new(), added }) }))
        }
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
