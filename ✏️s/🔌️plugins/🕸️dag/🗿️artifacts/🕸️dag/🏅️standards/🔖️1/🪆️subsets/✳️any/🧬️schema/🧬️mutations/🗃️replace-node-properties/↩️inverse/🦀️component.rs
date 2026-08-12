//! ↩️ Inverse for `ReplaceNodeProperties` — the OLD `properties` looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceNodeProperties, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::replace_node_properties(payload.id.clone(), node.properties.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
