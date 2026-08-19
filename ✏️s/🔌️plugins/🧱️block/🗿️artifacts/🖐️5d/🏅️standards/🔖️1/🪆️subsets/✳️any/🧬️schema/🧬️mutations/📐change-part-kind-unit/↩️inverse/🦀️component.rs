//! ↩️ Inverse for `ChangePartKindUnit` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangePartKindUnit, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_unit::mutation::change_part_kind_unit(base.part_kind.unit.clone())]
}
//#endregion 🔖️Inverse
