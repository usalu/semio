//! ↩️ Inverse for `RenamePartKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::RenamePartKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::rename_part_kind::rename_part_kind(base.part_kind.name.clone())]
}
//#endregion 🔖️Inverse
