//! ↩️ Inverse for `ChangePartKindVariant`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangePartKindVariant, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_part_kind_variant::change_part_kind_variant(base.part_kind.variant.clone())]
}
//#endregion 🔖️Inverse
