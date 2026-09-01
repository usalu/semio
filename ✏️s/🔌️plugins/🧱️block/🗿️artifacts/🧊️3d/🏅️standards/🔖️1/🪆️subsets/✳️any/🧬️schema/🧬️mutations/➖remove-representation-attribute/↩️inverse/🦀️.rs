//! ↩️ Inverse for `RemoveRepresentationAttribute`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemoveRepresentationAttribute, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    match existing.attributes.iter().find(|attribute| attribute.key == payload.key) {
        Some(attribute) => vec![super::super::add_representation_attribute::add_representation_attribute(payload.id.clone(), attribute.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
