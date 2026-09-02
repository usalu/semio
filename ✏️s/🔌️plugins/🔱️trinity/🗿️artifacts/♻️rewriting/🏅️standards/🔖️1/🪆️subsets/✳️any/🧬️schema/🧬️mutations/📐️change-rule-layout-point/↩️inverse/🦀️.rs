//! ↩️ Inverse for `ChangeRuleLayoutPoint` — the OLD point looked up from BASE: `change` back to it
//! if the key existed, `remove` if it was previously absent.
use crate::artifacts::rewriting::mutations::{change_rule_layout_point, remove_rule_layout_point, RewriteRuleMutation};
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeRuleLayoutPoint, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match base.rule_layout.get(&payload.key) {
        Some(old) => vec![change_rule_layout_point(payload.key.clone(), old.clone())],
        None => vec![remove_rule_layout_point(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
