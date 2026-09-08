//! ↩️ Inverse for `MoveNode` — restores the BASE position. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveNode, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::move_node::move_node(node.id.clone(), node.x, node.y)]
}
//#endregion 🔖️Inverse
