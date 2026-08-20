//! ↩️ `remove-node-property` — undo re-attaches the captured property at the same BASE-state
//! index; an absent `node_id`/`index` ⇒ `Vec::new()`.

use super::mutation::RemoveNodeProperty;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{add_node_property, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &RemoveNodeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id).and_then(|node| node.properties.get(payload.index)) {
        Some(property) => vec![SemioGraphMutation::AddNodeProperty(add_node_property::mutation::AddNodeProperty { node_id: payload.node_id.clone(), index: payload.index, property: property.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
