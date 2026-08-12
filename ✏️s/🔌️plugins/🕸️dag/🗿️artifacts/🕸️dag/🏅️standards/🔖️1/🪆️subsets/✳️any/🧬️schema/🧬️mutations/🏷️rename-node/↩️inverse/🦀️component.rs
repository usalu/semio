//! ↩️ Inverse for `RenameNode` — renames back to the id captured in BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameNode, base: &DagSnapshot) -> Vec<DagMutation> {
    if base.nodes.iter().any(|node| node.id == payload.id) {
        vec![crate::artifacts::dag::mutations::rename_node::mutation::rename_node(payload.new_id.clone(), payload.id.clone())]
    } else {
        Vec::new()
    }
}
//#endregion 🔖️Inverse
