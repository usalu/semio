//! ↩️ Inverse for `RemoveRepresentationTag`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveRepresentationTag, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if existing.tags.contains(&payload.tag) {
        vec![super::super::add_representation_tag::add_representation_tag(payload.id.clone(), payload.tag.clone())]
    } else {
        Vec::new()
    }
}
//#endregion 🔖️Inverse
