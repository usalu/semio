//! ↩️ Inverse for `AddNodeHandle` — always a `remove-node-handle` of the handle it added.
use crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::RemoveNodeHandle;
use crate::Puzzle2dSnapshot;
use crate::mirror::standards::v1::subsets::any::schema::mutations::add_node_handle::component::AddNodeHandle;

//#region 🔖️Inverse
pub fn inverse<Op: From<AddNodeHandle> + From<RemoveNodeHandle>>(payload: &super::AddNodeHandle, _base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::remove_node_handle(payload.node_id.clone(), payload.handle.id.clone())]

    })())
}
//#endregion 🔖️Inverse
