//! 🔺️ Diff for `DeleteNode`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::IndexedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some((parent, index)) = super::parent_and_index(&payload.at) else {
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
