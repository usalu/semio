//! ↩️ Inverse for `RemoveRepresentationTag`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemoveRepresentationTag, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
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
