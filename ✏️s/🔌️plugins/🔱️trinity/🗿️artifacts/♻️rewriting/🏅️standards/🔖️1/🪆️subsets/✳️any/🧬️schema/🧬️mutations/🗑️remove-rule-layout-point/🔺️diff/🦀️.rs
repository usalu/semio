//! 🔺️ Sparse diff builder for `RemoveRuleLayoutPoint` — `None` signals a clear.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;
use replication::MapDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveRuleLayoutPoint, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !base.rule_layout.contains_key(&payload.key) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Rule layout point \"{}\" is already absent.", payload.key));
    }
    let layout = MapDelta::remove(payload.key.clone());
    protocol::MutationOutcome::new(RewritingDiff { rule_layout: Some(layout), ..Default::default() })
}
//#endregion 🔖️Diff
