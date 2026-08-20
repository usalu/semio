//! ↩️ `delete-node` — undo re-creates the captured node at its BASE-state address; absent/root
//! address ⇒ `Vec::new()`.

use super::diff::parent_and_index;
use super::mutation::DeleteNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{create_node, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    let Some((parent, index)) = parent_and_index(&payload.at) else { return Vec::new() };
    match node_at(base, &parent) {
        Some(DrawNode::Group { children, .. }) => match children.get(index) {
            Some(node) => vec![SemioDrawingMutation::CreateNode(create_node::mutation::CreateNode { parent, index, node: node.clone() })],
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
