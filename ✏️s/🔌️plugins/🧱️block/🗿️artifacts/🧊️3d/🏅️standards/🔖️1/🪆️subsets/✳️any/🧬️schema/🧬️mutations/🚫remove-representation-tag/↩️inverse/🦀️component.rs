//! ↩️ Inverse for `RemoveRepresentationTag` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RemoveRepresentationTag, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else { return Vec::new(); };
    if existing.tags.contains(&payload.tag) { vec![super::super::add_representation_tag::mutation::add_representation_tag(payload.id.clone(), payload.tag.clone())] } else { Vec::new() }
}
//#endregion 🔖️Inverse
