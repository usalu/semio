//! ↩️ Inverse for `AddNodeProperty`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, remove_node_property};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddNodeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id) {
        Some(node) => {
            let at = payload.index.min(node.properties.len());
            vec![SemioGraphMutation::RemoveNodeProperty(remove_node_property::RemoveNodeProperty { node_id: payload.node_id.clone(), index: at })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
