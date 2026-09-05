//! ↩️ Inverse for `DeleteGrip`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DeleteGrip, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_grip::create_grip(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
