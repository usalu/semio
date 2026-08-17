//! ↩️ Inverse for `AddRepresentationAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddRepresentationAttribute, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_representation_attribute::mutation::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
