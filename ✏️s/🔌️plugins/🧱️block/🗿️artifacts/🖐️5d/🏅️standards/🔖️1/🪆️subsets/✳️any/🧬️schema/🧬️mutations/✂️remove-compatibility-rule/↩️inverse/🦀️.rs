//! ↩️ Inverse for `RemoveCompatibilityRule`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveCompatibilityRule, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.compatibility.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::add_compatibility_rule::add_compatibility_rule(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
