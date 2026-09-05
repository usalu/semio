//! ↩️ Inverse for `FlattenNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, unflatten_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::FlattenNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(node @ DrawNode::Group { children, .. }) if super::collect_flattened_leaves(children).is_some() => {
            vec![SemioDrawingMutation::UnflattenNode(unflatten_node::UnflattenNode { at: payload.at.clone(), original: node.clone() })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
