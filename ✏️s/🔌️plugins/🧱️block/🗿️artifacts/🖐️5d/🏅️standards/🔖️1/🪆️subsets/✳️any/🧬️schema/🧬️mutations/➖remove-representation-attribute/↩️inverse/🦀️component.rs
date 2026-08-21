//! ↩️ Inverse for `RemoveRepresentationAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveRepresentationAttribute, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    match existing.attributes.iter().find(|attribute| attribute.key == payload.key) {
        Some(attribute) => vec![super::super::add_representation_attribute::mutation::add_representation_attribute(payload.id.clone(), attribute.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
