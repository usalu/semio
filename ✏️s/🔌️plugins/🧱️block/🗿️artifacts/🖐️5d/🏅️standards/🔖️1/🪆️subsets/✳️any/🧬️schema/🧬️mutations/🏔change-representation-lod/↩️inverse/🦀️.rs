//! ↩️ Inverse for `ChangeRepresentationLod`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeRepresentationLod, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_lod::change_representation_lod(payload.id.clone(), existing.lod.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
