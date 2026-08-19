//! ↩️ Inverse for `ChangeNodeIcon` — restores the BASE field value on the addressed node. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeNodeIcon, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::change_node_icon::mutation::change_node_icon(node.id.clone(), node.icon_kind.clone())]
}
//#endregion 🔖️Inverse
