//! ↩️ Inverse for `RenameNodeKind`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::RenameNodeKind, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::rename_node_kind::rename_node_kind(base.node_kind.name.clone())]

    })())
}
//#endregion 🔖️Inverse
