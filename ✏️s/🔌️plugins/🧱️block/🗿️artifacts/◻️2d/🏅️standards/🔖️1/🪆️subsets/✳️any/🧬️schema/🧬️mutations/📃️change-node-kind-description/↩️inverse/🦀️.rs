//! ↩️ Inverse for `ChangeNodeKindDescription`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNodeKindDescription, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_node_kind_description::change_node_kind_description(base.node_kind.description.clone())]

    })())
}
//#endregion 🔖️Inverse
