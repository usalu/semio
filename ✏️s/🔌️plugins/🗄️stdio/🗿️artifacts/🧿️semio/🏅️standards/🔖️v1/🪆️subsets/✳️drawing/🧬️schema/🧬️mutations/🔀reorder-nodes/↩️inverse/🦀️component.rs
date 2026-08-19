//! ↩️ `reorder-nodes` — undo moves the node back: `reorder-nodes{parent, from: min(to,len-1), to:
//! from}` (`taxonomy.md` addressing convention #3); out-of-range BASE `from` or non-`Group`
//! `parent` ⇒ `Vec::new()`.

use super::mutation::ReorderNodes;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ReorderNodes, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if !children.is_empty() && payload.from < children.len() => {
            let landed_at = payload.to.min(children.len() - 1);
            vec![SemioDrawingMutation::ReorderNodes(ReorderNodes { parent: payload.parent.clone(), from: landed_at, to: payload.from })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
