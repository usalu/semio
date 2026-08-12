//! ↩️ Inverse for `ChangeNodeOperatorKind` — the OLD `operator_kind` looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeOperatorKind, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::change_node_operator_kind(payload.id.clone(), node.operator_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
