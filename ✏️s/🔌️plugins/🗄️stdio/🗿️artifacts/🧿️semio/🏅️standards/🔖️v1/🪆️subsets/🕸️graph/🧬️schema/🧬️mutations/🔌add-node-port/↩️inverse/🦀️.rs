//! ↩️ Inverse for `AddNodePort`.

use crate::standards::v1::subsets::graph::schema::mutations::{remove_node_port, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

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
