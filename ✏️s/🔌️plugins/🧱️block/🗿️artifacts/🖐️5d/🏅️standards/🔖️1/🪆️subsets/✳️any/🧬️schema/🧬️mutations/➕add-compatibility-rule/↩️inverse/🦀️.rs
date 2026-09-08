//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddCompatibilityRule, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
