//! 🔺️ `delete-node` — sparse diff construction; `at` being the layer root (empty `path`, no
//! parent to remove a child from), the parent not resolving to a `Group`, or the addressed index
//! being out of range are all `mutation.target-missing` (Error, empty diff).

use super::mutation::DeleteNode;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::IndexedTripleDiff;
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
pub fn diff(payload: &DeleteNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some((parent, index)) = parent_and_index(&payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} has no parent to delete from (layer root).", payload.at.layer), [payload.at.layer.to_string()]);
    };
    match node_at(base, &parent) {
        Some(DrawNode::Group { children, .. }) if index < children.len() => {
            protocol::MutationOutcome::new(diff_at_path(&parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) })))
        }
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
