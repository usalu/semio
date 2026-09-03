//! 🔺️ Diff for `GroupNodes`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::GroupNodes, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    if !super::is_contiguous_ascending(&payload.indices) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Indices for group in layer #{} are empty or not a contiguous ascending run.", payload.parent.layer), [payload.parent.layer.to_string()]);
    }
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.indices.iter().all(|&i| i < children.len()) => {
            let grouped: Vec<DrawNode> = payload.indices.iter().map(|&i| children[i].clone()).collect();
            let new_group = DrawNode::Group { transform: payload.transform, children: grouped };
            let at = payload.indices[0];
            protocol::MutationOutcome::new(diff_at_path(
                &payload.parent,
                DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: payload.indices.clone(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: new_group }] }) }),
            ))
        }
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Parent at layer #{} does not resolve to a group, or an index is out of range.", payload.parent.layer), [payload.parent.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
