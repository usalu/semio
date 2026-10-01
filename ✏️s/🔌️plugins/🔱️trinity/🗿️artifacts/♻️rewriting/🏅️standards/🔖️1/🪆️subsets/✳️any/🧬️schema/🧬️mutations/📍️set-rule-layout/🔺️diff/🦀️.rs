//! 🔺️ Sparse diff builder for `SetRuleLayoutPoints` — one map entry per key that changes: a set for a point that differs, a
//! removal for a cleared key the map holds.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::{LayoutPoint, RewritingSnapshot};
use replication::MapDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::SetRuleLayoutPoints, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "every rule node is named at most once, every point finite", payload.cleared.clone());
    }
    let mut layout = MapDelta::default();
    for placement in &payload.points {
        let point = LayoutPoint { x: placement.x, y: placement.y };
        if base.rule_layout.get(&placement.key) != Some(&point) {
            layout.absorb(MapDelta::set(placement.key.clone(), point));
        }
    }
    for key in payload.cleared.iter().filter(|key| base.rule_layout.contains_key(*key)) {
        layout.absorb(MapDelta::remove(key.clone()));
    }
    if layout.is_empty() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "every rule node already sits where the payload places it");
    }
    protocol::MutationOutcome::new(RewritingDiff { rule_layout: Some(layout), ..Default::default() })
}
//#endregion 🔖️Diff
