//! ↩️ `change-node-label` — undo sets `label` back to the BASE-state value; an absent `id` ⇒
//! `Vec::new()`.

use super::mutation::ChangeNodeLabel;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeNodeLabel, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: payload.id.clone(), new_label: node.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
