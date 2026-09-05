//! ↩️ Inverse for `ChangePartKindIcon`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangePartKindIcon, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_icon::change_part_kind_icon(base.part_kind.icon.clone())]
}
//#endregion 🔖️Inverse
