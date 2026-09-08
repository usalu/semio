//! ↩️ Inverse for `CreateRepresentation`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateRepresentation, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_representation::delete_representation(payload.representation.id.clone())]
}
//#endregion 🔖️Inverse
