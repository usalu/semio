//! ↩️ Inverse for `DeleteNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, create_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    let Some((parent, index)) = super::parent_and_index(&payload.at) else { return Vec::new() };
    match node_at(base, &parent) {
        Some(DrawNode::Group { children, .. }) => match children.get(index) {
            Some(node) => vec![SemioDrawingMutation::CreateNode(create_node::CreateNode { parent, index, node: node.clone() })],
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
