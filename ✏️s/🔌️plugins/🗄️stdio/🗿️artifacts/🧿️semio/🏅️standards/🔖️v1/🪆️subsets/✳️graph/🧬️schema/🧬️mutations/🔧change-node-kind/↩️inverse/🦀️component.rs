//! ↩️ `change-node-kind` — undo sets `kind` back to the BASE-state value; an absent `id` ⇒
//! `Vec::new()`.

use super::mutation::ChangeNodeKind;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeNodeKind, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![SemioGraphMutation::ChangeNodeKind(ChangeNodeKind { id: payload.id.clone(), new_kind: node.kind.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
