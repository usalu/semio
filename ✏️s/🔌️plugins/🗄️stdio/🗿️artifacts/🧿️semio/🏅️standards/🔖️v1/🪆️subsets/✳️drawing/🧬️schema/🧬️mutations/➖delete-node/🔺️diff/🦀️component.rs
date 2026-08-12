//! 🔺️ `delete-node` — sparse diff construction; a no-op when `at` is the layer root (empty
//! `path`) or the addressed index is out of range.

use super::mutation::DeleteNode;
use crate::artifacts::semio::standards::v1::engine::triples::IndexedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️ParentSplit
/// ✂️️ Splits `at` into (parent address, own index within the parent's `children`) -- `None` for
/// the layer root (empty `path`), which has no parent to remove a child from.
pub(crate) fn parent_and_index(at: &NodePath) -> Option<(NodePath, usize)> {
    let mut parent_path = at.path.clone();
    let index = parent_path.pop()?;
    Some((NodePath { layer: at.layer, path: parent_path }, index))
}
//#endregion 🔖️ParentSplit

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    let Some((parent, index)) = parent_and_index(&payload.at) else { return SemioDrawingDiff::default() };
    match node_at(base, &parent) {
        Some(DrawNode::Group { children, .. }) if index < children.len() => {
            diff_at_path(&parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }))
        }
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
