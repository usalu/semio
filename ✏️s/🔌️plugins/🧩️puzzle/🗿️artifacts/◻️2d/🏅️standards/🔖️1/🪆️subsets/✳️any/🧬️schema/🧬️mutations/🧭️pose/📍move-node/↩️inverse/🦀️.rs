//! ↩️ Inverse for `MoveNode` — restores the BASE position. Missing target ⇒ `Vec::new()`.
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::move_node::component::MoveNode;

//#region 🔖️Inverse
pub fn inverse<Op: From<MoveNode>>(payload: &super::MoveNode, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::move_node::move_node(node.id.clone(), node.x, node.y)]

    })())
}
//#endregion 🔖️Inverse
