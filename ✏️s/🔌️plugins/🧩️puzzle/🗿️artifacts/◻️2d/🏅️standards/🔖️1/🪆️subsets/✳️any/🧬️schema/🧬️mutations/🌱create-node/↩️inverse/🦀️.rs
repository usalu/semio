//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateNode, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_node::delete_node(payload.node.id.clone())]
}
//#endregion 🔖️Inverse
