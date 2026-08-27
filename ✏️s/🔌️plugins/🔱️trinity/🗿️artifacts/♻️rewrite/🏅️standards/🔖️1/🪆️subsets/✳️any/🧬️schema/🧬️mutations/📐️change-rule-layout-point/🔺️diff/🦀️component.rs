//! 🔺️ Sparse diff builder for `ChangeRuleLayoutPoint`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRuleLayoutPoint, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if base.rule_layout.get(&payload.key) == Some(&payload.new_point) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Rule layout point \"{}\" is unchanged.", payload.key));
    }
    let mut layout = BTreeMap::new();
    layout.insert(payload.key.clone(), Some(payload.new_point.clone()));
    protocol::MutationOutcome::new(RewriteDiff { rule_layout: Some(layout), ..Default::default() })
}
//#endregion 🔖️Diff
