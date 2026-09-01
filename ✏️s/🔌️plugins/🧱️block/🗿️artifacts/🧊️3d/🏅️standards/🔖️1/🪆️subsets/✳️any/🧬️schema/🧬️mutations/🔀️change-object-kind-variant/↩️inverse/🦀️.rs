//! ↩️ Inverse for `ChangeObjectKindVariant`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeObjectKindVariant, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_variant::change_object_kind_variant(base.object_kind.variant.clone())]
}
//#endregion 🔖️Inverse
