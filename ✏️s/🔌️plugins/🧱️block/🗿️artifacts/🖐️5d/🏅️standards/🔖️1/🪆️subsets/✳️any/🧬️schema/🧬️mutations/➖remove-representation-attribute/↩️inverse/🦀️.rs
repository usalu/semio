//! ↩️ Inverse for `RemoveRepresentationAttribute`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveRepresentationAttribute, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    match existing.attributes.iter().find(|attribute| attribute.key == payload.key) {
        Some(attribute) => vec![super::super::add_representation_attribute::add_representation_attribute(payload.id.clone(), attribute.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
