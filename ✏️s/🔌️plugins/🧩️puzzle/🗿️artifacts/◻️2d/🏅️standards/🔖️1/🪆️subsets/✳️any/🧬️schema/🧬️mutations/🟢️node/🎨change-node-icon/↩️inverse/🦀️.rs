//! 🔘️ target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeNodeIcon` — restores the BASE field value on the addressed node. Missing
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_node_icon::component::ChangeNodeIcon;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeNodeIcon>>(payload: &super::ChangeNodeIcon, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_node_icon::change_node_icon(node.id.clone(), node.icon_kind.clone())]

    })())
}
//#endregion 🔖️Inverse
