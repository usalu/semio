//! ↩️ `create-node` — undo is `delete-node` at the (clamped) FINAL-state address the node landed
//! at; a no-op when `parent` does not resolve to a `Group`.

use super::mutation::CreateNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{node_at, NodePath};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{delete_node, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreateNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) => {
            let at = payload.index.min(children.len());
            let mut path = payload.parent.path.clone();
            path.push(at);
            vec![SemioDrawingMutation::DeleteNode(delete_node::mutation::DeleteNode { at: NodePath { layer: payload.parent.layer, path } })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
