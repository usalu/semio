//! ↩️ Inverse for `RenamePartKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::RenamePartKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::rename_part_kind::mutation::rename_part_kind(base.part_kind.name.clone())]
}
//#endregion 🔖️Inverse
