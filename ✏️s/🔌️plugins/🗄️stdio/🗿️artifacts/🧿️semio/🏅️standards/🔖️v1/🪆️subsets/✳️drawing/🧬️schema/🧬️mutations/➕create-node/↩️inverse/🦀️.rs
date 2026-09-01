//! ↩️ Inverse for `CreateNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, delete_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) => {
            let at = payload.index.min(children.len());
            let mut path = payload.parent.path.clone();
            path.push(at);
            vec![SemioDrawingMutation::DeleteNode(delete_node::DeleteNode { at: NodePath { layer: payload.parent.layer, path } })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
