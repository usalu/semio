//! ↩️ Inverse for `AddRepresentationAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::AddRepresentationAttribute, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_representation_attribute::mutation::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
