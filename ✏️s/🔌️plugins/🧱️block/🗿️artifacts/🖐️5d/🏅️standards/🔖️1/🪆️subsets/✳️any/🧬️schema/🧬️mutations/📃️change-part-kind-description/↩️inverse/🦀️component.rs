//! ↩️ Inverse for `ChangePartKindDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangePartKindDescription, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_description::mutation::change_part_kind_description(base.part_kind.description.clone())]
}
//#endregion 🔖️Inverse
