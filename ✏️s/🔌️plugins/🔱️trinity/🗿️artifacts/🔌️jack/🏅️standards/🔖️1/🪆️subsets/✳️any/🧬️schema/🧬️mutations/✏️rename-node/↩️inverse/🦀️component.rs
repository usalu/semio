//! ↩️ Inverse for `RenameNode` — the OLD `name` looked up from BASE. Missing target ⇒ `Vec::new()`.
use crate::artifacts::jack::mutations::{rename_node, TrinityGraphMutation};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameNode, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    base.nodes.iter().find(|node| node.id == payload.id).map(|node| vec![rename_node(payload.id.clone(), node.name.clone())]).unwrap_or_default()
}
//#endregion 🔖️Inverse
