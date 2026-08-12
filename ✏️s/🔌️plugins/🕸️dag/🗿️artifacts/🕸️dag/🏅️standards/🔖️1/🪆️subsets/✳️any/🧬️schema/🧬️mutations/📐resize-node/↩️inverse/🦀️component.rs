//! ↩️ Inverse for `ResizeNode` — the OLD `(width, height)` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ResizeNode, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::resize_node(payload.id.clone(), node.width, node.height)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
