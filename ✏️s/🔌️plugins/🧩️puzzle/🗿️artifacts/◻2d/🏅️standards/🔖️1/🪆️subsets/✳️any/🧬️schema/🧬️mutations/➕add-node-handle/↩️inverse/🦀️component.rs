//! ↩️ Inverse for `AddNodeHandle` — always a `remove-node-handle` of the handle it added.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::AddNodeHandle, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::artifacts::puzzle2d::mutations::remove_node_handle::mutation::remove_node_handle(payload.node_id.clone(), payload.handle.id.clone())]
}
//#endregion 🔖️Inverse
