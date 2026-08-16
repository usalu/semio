//! 🔺️ Sparse diff builder for `RemoveRuleLayoutPoint` — `None` signals a clear.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveRuleLayoutPoint, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if !base.rule_layout.contains_key(&payload.key) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Rule layout point \"{}\" is already absent.", payload.key));
    }
    let mut layout = BTreeMap::new();
    layout.insert(payload.key.clone(), None);
    protocol::MutationOutcome::new(RewriteDiff { rule_layout: Some(layout), ..Default::default() })
}
//#endregion 🔖️Diff
