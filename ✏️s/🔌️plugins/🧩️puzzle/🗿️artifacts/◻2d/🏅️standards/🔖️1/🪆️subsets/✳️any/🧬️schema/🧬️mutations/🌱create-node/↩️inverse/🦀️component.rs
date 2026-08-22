//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the id it created.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateNode, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::artifacts::puzzle2d::mutations::delete_node::mutation::delete_node(payload.node.id.clone())]
}
//#endregion 🔖️Inverse
