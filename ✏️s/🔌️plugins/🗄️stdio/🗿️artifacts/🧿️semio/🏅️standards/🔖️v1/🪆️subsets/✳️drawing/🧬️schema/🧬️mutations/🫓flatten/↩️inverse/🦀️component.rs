//! ↩️ `flatten` — undo is `unflatten` carrying the BASE-state subtree wholesale; a no-op for the
//! same shapes `diff` refuses.

use super::diff::collect_flattened_leaves;
use super::mutation::FlattenNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{unflatten, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &FlattenNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(node @ DrawNode::Group { children, .. }) if collect_flattened_leaves(children).is_some() => {
            vec![SemioDrawingMutation::Unflatten(unflatten::mutation::UnflattenNode { at: payload.at.clone(), original: node.clone() })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
