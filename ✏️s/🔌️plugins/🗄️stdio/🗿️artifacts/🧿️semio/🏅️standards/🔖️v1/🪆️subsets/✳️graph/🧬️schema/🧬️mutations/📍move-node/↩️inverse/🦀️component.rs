//! ↩️ `move-node` — undo sets `position` back to the BASE-state value; an absent `id` ⇒
//! `Vec::new()`.

use super::mutation::MoveNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &MoveNode, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![SemioGraphMutation::MoveNode(MoveNode { id: payload.id.clone(), new_position: node.position.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
