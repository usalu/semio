//! ↩️ Inverse for `MoveNode` — the OLD `(x, y)` looked up from BASE. Missing target ⇒ `Vec::new()`.
use crate::artifacts::jack::mutations::{move_node, TrinityGraphMutation};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveNode, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    base.nodes().iter().find(|node| node.id == payload.id).map(|node| vec![move_node(payload.id.clone(), node.x, node.y)]).unwrap_or_default()
}
//#endregion 🔖️Inverse
