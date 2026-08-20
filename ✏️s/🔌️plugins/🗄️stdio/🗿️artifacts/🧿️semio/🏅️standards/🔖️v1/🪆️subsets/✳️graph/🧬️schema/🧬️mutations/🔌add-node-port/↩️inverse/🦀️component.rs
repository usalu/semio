//! ↩️ `add-node-port` — undo is `remove-node-port` at the (clamped) FINAL-state index the port
//! landed at, which is also a valid BASE-state index for the follow-up removal; an absent
//! `node_id` ⇒ `Vec::new()`.

use super::mutation::AddNodePort;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{remove_node_port, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &AddNodePort, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.node_id) {
        Some(node) => {
            let at = payload.index.min(node.ports.len());
            vec![SemioGraphMutation::RemoveNodePort(remove_node_port::mutation::RemoveNodePort { node_id: payload.node_id.clone(), index: at })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
