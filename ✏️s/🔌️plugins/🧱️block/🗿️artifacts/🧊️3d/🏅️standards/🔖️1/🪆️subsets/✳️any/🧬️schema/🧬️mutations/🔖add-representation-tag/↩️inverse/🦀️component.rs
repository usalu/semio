//! ↩️ Inverse for `AddRepresentationTag` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::AddRepresentationTag, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_representation_tag::mutation::remove_representation_tag(payload.id.clone(), payload.tag.clone())]
}
//#endregion 🔖️Inverse
