//! ↩️ Inverse for `DeleteGrip` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteGrip, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::create_grip::mutation::create_grip(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
