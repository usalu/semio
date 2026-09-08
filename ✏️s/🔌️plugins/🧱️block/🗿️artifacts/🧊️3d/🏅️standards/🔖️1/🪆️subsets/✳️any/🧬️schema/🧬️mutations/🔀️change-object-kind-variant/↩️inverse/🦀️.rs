//! ↩️ Inverse for `ChangeObjectKindVariant`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindVariant, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_variant::change_object_kind_variant(base.object_kind.variant.clone())]
}
//#endregion 🔖️Inverse
