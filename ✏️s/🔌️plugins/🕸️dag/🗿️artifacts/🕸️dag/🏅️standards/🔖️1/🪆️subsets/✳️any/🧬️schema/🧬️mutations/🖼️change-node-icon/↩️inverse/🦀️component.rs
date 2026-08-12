//! ↩️ Inverse for `ChangeNodeIcon` — the OLD `icon` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeIcon, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::change_node_icon(payload.id.clone(), node.icon.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
