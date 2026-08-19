//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateNode, _base: &DagSnapshot) -> Vec<DagMutation> {
    vec![crate::artifacts::dag::mutations::delete_node::mutation::delete_node(payload.node.id.clone())]
}
//#endregion 🔖️Inverse
