//! ↩️ Inverse for `ChangeNodeName` — the OLD `name` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeName, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::change_node_name(payload.id.clone(), node.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
