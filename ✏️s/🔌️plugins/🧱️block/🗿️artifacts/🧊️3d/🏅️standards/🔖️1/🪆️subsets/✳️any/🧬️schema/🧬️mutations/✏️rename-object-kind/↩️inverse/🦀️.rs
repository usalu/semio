//! ↩️ Inverse for `RenameObjectKind`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::RenameObjectKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::rename_object_kind::rename_object_kind(base.object_kind.name.clone())]
}
//#endregion 🔖️Inverse
