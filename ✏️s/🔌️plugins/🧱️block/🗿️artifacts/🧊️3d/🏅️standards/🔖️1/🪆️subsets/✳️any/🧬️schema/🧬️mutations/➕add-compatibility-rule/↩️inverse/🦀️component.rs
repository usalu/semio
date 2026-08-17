//! ↩️ Inverse for `AddCompatibilityRule` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddCompatibilityRule, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_compatibility_rule::mutation::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
