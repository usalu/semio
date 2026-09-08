//! 🔺️ Sparse diff builder for `ChangeRuleLayoutPoint`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRuleLayoutPoint, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.rule_layout.get(&payload.key) == Some(&payload.new_point) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Rule layout point \"{}\" is unchanged.", payload.key));
    }
    let mut layout = BTreeMap::new();
    layout.insert(payload.key.clone(), Some(payload.new_point));
    protocol::MutationOutcome::new(RewritingDiff { rule_layout: Some(layout), ..Default::default() })
}
//#endregion 🔖️Diff
