//! 🔘️ target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeNodeKind` — restores the BASE field value on the addressed node. Missing
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind::component::ChangeNodeKind;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeNodeKind>>(payload: &super::ChangeNodeKind, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_node_kind::change_node_kind(node.id.clone(), node.node_kind.clone())]

    })())
}
//#endregion 🔖️Inverse
