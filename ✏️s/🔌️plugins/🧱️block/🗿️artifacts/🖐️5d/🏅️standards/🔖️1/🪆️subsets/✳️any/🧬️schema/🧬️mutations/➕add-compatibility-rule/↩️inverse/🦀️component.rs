//! ↩️ Inverse for `AddCompatibilityRule` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddCompatibilityRule, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_compatibility_rule::mutation::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
