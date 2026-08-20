//! ↩️ `remove-node-port` — undo re-attaches the captured port at the same BASE-state index; an
//! absent `node_id`/`index` ⇒ `Vec::new()`.

use super::mutation::RemoveNodePort;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{add_node_port, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &RemoveNodePort, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id).and_then(|node| node.ports.get(payload.index)) {
        Some(port) => vec![SemioGraphMutation::AddNodePort(add_node_port::mutation::AddNodePort { node_id: payload.node_id.clone(), index: payload.index, port: port.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
