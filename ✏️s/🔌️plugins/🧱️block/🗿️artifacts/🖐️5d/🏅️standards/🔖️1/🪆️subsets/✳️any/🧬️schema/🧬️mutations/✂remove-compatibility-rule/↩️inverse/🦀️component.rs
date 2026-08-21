//! ↩️ Inverse for `RemoveCompatibilityRule` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveCompatibilityRule, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.compatibility.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::add_compatibility_rule::mutation::add_compatibility_rule(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
