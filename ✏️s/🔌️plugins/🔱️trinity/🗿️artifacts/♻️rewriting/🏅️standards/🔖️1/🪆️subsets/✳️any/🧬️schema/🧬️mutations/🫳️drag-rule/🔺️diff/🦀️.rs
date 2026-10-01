//! 🔺️ Sparse diff builder for `DragRuleNodes` — every addressed rule-graph node's layout point set to its BASE position (layout
//! point, else default slot) plus the offset, one map entry per moved node.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::rule_graph_position;
use crate::{LayoutPoint, RewritingSnapshot};
use replication::MapDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::DragRuleNodes, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag names at least one rule node, never one twice, by a finite offset", payload.targets.clone());
    }
    let placed: Vec<(String, LayoutPoint)> = payload.targets.iter().filter_map(|id| rule_graph_position(base, id).map(|point| (id.clone(), point))).collect();
    let missing: Vec<String> = payload.targets.iter().filter(|id| !placed.iter().any(|(placed, _)| placed == *id)).cloned().collect();
    if placed.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("the rule draws none of the {} target node(s)", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (no node of this rule): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "a zero offset moves nothing").at(payload.targets.clone())]));
    }
    let mut layout = MapDelta::default();
    for (id, point) in placed {
        layout.absorb(MapDelta::set(id, LayoutPoint { x: point.x + payload.dx, y: point.y + payload.dy }));
    }
    protocol::MutationOutcome::new(RewritingDiff { rule_layout: Some(layout), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
