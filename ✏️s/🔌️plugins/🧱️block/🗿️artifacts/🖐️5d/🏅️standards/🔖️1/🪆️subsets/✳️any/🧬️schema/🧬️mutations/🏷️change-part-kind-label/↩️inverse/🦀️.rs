//! ↩️ Inverse for `ChangePartKindLabel`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangePartKindLabel, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_label::change_part_kind_label(base.part_kind.label.clone())]
}
//#endregion 🔖️Inverse
