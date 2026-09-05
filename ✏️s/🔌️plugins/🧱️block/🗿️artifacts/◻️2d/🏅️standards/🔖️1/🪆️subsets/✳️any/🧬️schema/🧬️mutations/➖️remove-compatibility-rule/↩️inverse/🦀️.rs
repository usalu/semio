//! ↩️ Inverse for `RemoveCompatibilityRule`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveCompatibilityRule, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.compatibility.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::add_compatibility_rule::add_compatibility_rule(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
