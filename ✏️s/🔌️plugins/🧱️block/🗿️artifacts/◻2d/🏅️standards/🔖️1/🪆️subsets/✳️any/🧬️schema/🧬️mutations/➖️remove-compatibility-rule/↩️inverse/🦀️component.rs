//! ↩️ Inverse for `RemoveCompatibilityRule` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveCompatibilityRule, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.compatibility.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::add_compatibility_rule::mutation::add_compatibility_rule(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
