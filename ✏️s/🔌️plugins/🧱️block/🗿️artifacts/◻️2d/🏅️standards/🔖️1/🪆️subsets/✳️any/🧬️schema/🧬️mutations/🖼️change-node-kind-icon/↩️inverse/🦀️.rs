//! ↩️ Inverse for `ChangeNodeKindIcon`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNodeKindIcon, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_node_kind_icon::change_node_kind_icon(base.node_kind.icon.clone())]

    })())
}
//#endregion 🔖️Inverse
