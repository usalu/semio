//! ↩️ Inverse for `MoveNode`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::MoveNode, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![SemioGraphMutation::MoveNode(super::MoveNode { id: payload.id.clone(), new_position: node.position.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
