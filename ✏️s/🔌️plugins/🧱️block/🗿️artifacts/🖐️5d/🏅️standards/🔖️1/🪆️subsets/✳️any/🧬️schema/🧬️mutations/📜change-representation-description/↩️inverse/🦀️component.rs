//! ↩️ Inverse for `ChangeRepresentationDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeRepresentationDescription, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_representation_description::mutation::change_representation_description(payload.id.clone(), existing.description.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
