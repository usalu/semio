//! ↩️ Inverse for `CreateRepresentation`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateRepresentation, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_representation::delete_representation(payload.representation.id.clone())]
}
//#endregion 🔖️Inverse
