//! ↩️ Inverse for `AddCompatibilityRule` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::AddCompatibilityRule, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::remove_compatibility_rule::mutation::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
