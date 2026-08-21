//! ↩️ Inverse for `DeleteRepresentation` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DeleteRepresentation, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_representation::mutation::create_representation(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
