//! ↩️ Inverse for `ChangeNodeAnchor` — restores the BASE field value on the addressed node. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeNodeAnchor, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::change_node_anchor::mutation::change_node_anchor(node.id.clone(), node.anchor)]
}
//#endregion 🔖️Inverse
