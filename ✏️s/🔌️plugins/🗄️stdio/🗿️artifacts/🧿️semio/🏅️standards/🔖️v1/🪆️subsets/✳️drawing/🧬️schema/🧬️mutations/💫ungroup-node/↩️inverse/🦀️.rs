//! ↩️ Inverse for `UngroupNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, group_nodes};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::delete_node::parent_and_index;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::UngroupNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    let Some((parent, group_index)) = parent_and_index(&payload.at) else { return Vec::new() };
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, children }) => {
            let indices: Vec<usize> = (group_index..group_index + children.len()).collect();
            vec![SemioDrawingMutation::GroupNodes(group_nodes::GroupNodes { parent, indices, transform: *transform })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
