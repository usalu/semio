//! ↩️ `add-node-property` — undo is `remove-node-property` at the (clamped) FINAL-state index the
//! property landed at, which is also a valid BASE-state index for the follow-up removal; an
//! absent `node_id` ⇒ `Vec::new()`.

use super::mutation::AddNodeProperty;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{remove_node_property, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddNodeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id) {
        Some(node) => {
            let at = payload.index.min(node.properties.len());
            vec![SemioGraphMutation::RemoveNodeProperty(remove_node_property::mutation::RemoveNodeProperty { node_id: payload.node_id.clone(), index: at })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
