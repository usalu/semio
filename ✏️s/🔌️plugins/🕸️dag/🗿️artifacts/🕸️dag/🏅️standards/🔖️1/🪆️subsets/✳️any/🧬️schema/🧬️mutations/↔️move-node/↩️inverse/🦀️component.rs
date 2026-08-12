//! ↩️ Inverse for `MoveNode` — the OLD `(x, y)` looked up from BASE. Missing target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::MoveNode, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::move_node(payload.id.clone(), node.x, node.y)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
