//! ↩️ Inverse for `ChangePartKindVariant`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangePartKindVariant, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_variant::change_part_kind_variant(base.part_kind.variant.clone())]
}
//#endregion 🔖️Inverse
