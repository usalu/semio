//! ↩️ Inverse for `ChangePartKindDescription`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangePartKindDescription, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_description::change_part_kind_description(base.part_kind.description.clone())]
}
//#endregion 🔖️Inverse
