//! ↩️ Inverse for `ChangeRepresentationLod` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeRepresentationLod, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_representation_lod::mutation::change_representation_lod(payload.id.clone(), existing.lod.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
