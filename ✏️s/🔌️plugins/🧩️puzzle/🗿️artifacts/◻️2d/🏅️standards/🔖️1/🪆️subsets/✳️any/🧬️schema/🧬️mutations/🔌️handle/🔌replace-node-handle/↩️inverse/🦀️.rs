//! 🔁️ `Vec::new()`.
//! ↩️ Inverse for `ReplaceNodeHandle` — restores the BASE handle payload. Missing target ⇒
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::component::ReplaceNodeHandle;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceNodeHandle>>(payload: &super::ReplaceNodeHandle, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return Vec::new();
    };
    let Some(handle) = node.handles.iter().find(|handle| handle.id == payload.handle_id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::replace_node_handle(payload.node_id.clone(), payload.handle_id.clone(), handle.clone())]

    })())
}
//#endregion 🔖️Inverse
