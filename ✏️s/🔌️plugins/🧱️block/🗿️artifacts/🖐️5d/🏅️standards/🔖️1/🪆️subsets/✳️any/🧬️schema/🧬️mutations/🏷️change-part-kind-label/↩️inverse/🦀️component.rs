//! ↩️ Inverse for `ChangePartKindLabel` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangePartKindLabel, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_label::mutation::change_part_kind_label(base.part_kind.label.clone())]
}
//#endregion 🔖️Inverse
