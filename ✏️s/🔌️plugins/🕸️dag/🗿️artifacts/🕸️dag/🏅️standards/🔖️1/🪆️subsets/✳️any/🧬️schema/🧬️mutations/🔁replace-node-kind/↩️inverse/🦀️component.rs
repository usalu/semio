//! ↩️ Inverse for `ReplaceNodeKind` — the OLD `kind` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceNodeKind, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::replace_node_kind(payload.id.clone(), node.kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
