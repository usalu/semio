//! ↩️ Inverse for `ChangePartKindIcon` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangePartKindIcon, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_icon::mutation::change_part_kind_icon(base.part_kind.icon.clone())]
}
//#endregion 🔖️Inverse
