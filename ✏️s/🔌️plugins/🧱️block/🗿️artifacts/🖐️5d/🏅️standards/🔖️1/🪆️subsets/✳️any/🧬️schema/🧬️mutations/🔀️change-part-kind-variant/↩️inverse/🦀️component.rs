//! ↩️ Inverse for `ChangePartKindVariant` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangePartKindVariant, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_variant::mutation::change_part_kind_variant(base.part_kind.variant.clone())]
}
//#endregion 🔖️Inverse
