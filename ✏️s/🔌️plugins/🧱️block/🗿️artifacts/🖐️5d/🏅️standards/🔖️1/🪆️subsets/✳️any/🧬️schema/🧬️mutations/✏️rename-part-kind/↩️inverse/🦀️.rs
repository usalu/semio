//! ↩️ Inverse for `RenamePartKind`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::RenamePartKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::rename_part_kind::rename_part_kind(base.part_kind.name.clone())]
}
//#endregion 🔖️Inverse
