//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::mutations::{delete_node, TrinityGraphMutation};
use crate::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateNode, _base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    vec![delete_node(payload.node.id.clone())]
}
//#endregion 🔖️Inverse
