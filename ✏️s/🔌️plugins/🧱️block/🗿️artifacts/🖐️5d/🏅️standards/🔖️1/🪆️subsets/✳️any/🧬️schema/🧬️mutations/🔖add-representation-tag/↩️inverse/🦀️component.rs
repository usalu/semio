//! ↩️ Inverse for `AddRepresentationTag` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddRepresentationTag, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_representation_tag::mutation::remove_representation_tag(payload.id.clone(), payload.tag.clone())]
}
//#endregion 🔖️Inverse
