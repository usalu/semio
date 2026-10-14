//! ↩️ Inverse for `CreateNode` — always a `delete-node` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateNode> + From<DeleteNode>>(payload: &super::CreateNode, _base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_node::delete_node(payload.node.id.clone())]

    })())
}
//#endregion 🔖️Inverse
