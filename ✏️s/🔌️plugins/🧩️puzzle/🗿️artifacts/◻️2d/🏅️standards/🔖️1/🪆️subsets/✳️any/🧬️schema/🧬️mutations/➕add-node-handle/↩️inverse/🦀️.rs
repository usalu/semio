//! ↩️ Inverse for `AddNodeHandle` — always a `remove-node-handle` of the handle it added.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddNodeHandle, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::remove_node_handle(payload.node_id.clone(), payload.handle.id.clone())]
}
//#endregion 🔖️Inverse
