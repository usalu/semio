//! ↩️ Inverse for `ChangePartKindUnit`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangePartKindUnit, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_unit::change_part_kind_unit(base.part_kind.unit.clone())]
}
//#endregion 🔖️Inverse
