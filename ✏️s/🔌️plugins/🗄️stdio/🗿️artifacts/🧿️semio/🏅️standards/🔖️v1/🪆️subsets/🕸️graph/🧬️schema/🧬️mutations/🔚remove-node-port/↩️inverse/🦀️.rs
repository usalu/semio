//! ↩️ Inverse for `RemoveNodePort`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, add_node_port};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveNodePort, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id).and_then(|node| node.ports.get(payload.index)) {
        Some(port) => vec![SemioGraphMutation::AddNodePort(add_node_port::AddNodePort { node_id: payload.node_id.clone(), index: payload.index, port: port.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
