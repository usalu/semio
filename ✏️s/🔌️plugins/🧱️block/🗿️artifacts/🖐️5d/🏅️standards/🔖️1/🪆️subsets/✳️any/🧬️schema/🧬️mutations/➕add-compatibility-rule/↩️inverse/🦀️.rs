//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::AddCompatibilityRule, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
