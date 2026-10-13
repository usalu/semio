//! 🔺️ Diff for `UngroupNode`.

use semio_s_artifact_stdio_semio_base::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::schema::mutations::delete_node::parent_and_index;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::UngroupNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
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
