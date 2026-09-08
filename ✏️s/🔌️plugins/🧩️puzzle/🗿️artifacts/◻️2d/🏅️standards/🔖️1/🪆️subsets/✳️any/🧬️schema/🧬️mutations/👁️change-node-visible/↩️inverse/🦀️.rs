//! ↩️ Inverse for `ChangeNodeVisible` — restores the BASE field value on the addressed node. Missing
//! target ⇒ `Vec::new()`.
use crate::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeNodeVisible, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_node_visible::change_node_visible(node.id.clone(), node.visible)]
}
//#endregion 🔖️Inverse
