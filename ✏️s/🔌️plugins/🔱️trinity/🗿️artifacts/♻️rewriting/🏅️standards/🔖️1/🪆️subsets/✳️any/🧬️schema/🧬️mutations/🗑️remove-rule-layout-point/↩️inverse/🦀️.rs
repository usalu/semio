//! ↩️ Inverse for `RemoveRuleLayoutPoint` — the OLD point looked up from BASE, restored via
//! `change-rule-layout-point`. Missing key ⇒ `Vec::new()`.
use crate::artifacts::rewriting::mutations::{change_rule_layout_point, RewriteRuleMutation};
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveRuleLayoutPoint, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    base.rule_layout.get(&payload.key).map(|old| vec![change_rule_layout_point(payload.key.clone(), *old)]).unwrap_or_default()
}
//#endregion 🔖️Inverse
