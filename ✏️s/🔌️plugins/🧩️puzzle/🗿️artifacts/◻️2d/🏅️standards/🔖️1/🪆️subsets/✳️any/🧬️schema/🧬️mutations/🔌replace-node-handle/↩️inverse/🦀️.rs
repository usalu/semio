//! ↩️ Inverse for `ReplaceNodeHandle` — restores the BASE handle payload. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplaceNodeHandle, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return Vec::new();
    };
    let Some(handle) = node.handles.iter().find(|handle| handle.id == payload.handle_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::replace_node_handle::replace_node_handle(payload.node_id.clone(), payload.handle_id.clone(), handle.clone())]
}
//#endregion 🔖️Inverse
