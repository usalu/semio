//! ↩️ Inverse for `ChangeObjectKindVariant` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeObjectKindVariant, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_variant::mutation::change_object_kind_variant(base.object_kind.variant.clone())]
}
//#endregion 🔖️Inverse
