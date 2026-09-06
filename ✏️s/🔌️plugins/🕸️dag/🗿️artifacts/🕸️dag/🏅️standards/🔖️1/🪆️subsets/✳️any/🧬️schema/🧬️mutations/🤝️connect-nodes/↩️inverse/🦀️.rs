//! ↩️ Inverse for `ConnectNodes` — always a `disconnect-nodes` of the edge id it created.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ConnectNodes, _base: &DagSnapshot) -> Vec<DagMutation> {
    vec![crate::artifacts::dag::mutations::disconnect_nodes::mutation::disconnect_nodes(payload.id.clone())]
}
//#endregion 🔖️Inverse
