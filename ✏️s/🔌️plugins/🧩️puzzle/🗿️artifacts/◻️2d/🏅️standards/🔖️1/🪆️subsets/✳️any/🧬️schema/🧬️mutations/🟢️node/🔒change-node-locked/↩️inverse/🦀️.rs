//! 🔘️ target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeNodeLocked` — restores the BASE field value on the addressed node. Missing
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_node_locked::component::ChangeNodeLocked;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeNodeLocked>>(payload: &super::ChangeNodeLocked, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_node_locked::change_node_locked(node.id.clone(), node.locked)]

    })())
}
//#endregion 🔖️Inverse
