//! ↩️ Inverse for `ChangeNodeAbbreviation` — the OLD `abbreviation` looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeAbbreviation, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::change_node_abbreviation(payload.id.clone(), node.abbreviation.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
