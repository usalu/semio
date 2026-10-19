//! target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeNodeRoot` — restores the BASE field value on the addressed node. Missing
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeNodeRoot>>(payload: &super::ChangeNodeRoot, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_node_root::change_node_root(node.id.clone(), node.root)]

    })())
}
//#endregion 🔖️Inverse
