//! ↩️ Inverse for `AddNodePort`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, remove_node_port};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphPort, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddNodePort, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id) {
        Some(node) => {
            let at = payload.index.min(node.ports.len());
            vec![SemioGraphMutation::RemoveNodePort(remove_node_port::RemoveNodePort { node_id: payload.node_id.clone(), index: at })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
