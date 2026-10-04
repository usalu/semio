//! 🔺️ Diff for `DragNodes`.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
/// 🧮️ Every addressed node moves by `(dx, dy)` read off its BASE position. An empty or repeated target list or a non-finite
/// offset is a Fatal `mutation.invariant`, no addressed node left is `mutation.target-missing`, a node the graph lacks is
/// skipped as `mutation.partial`, a zero offset is `mutation.no-op`.
pub fn diff(payload: &super::DragNodes, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let ids: Vec<String> = payload.targets.iter().map(|id| id.value.clone()).collect();
    if ids.is_empty() || ids.iter().enumerate().any(|(at, id)| ids[..at].contains(id)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "targets must name at least one node and never one twice", ids);
    }
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", ids);
    }
    let missing: Vec<String> = payload.targets.iter().filter(|id| !base.nodes.iter().any(|node| node.id == **id)).map(|id| id.value.clone()).collect();
    if missing.len() == ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a node of this graph", ids.len()), ids);
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped (not in this graph): {}", missing.len(), ids.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::new(SemioGraphDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "a zero offset moves nothing").at(ids)]));
    }
    let mut nodes = base.nodes.clone();
    for node in nodes.iter_mut().filter(|node| payload.targets.contains(&node.id)) {
        node.position = SemioPoint2 { x: node.position.x + payload.dx, y: node.position.y + payload.dy };
    }
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }).absorb_messages(partial)
}
//#endregion 🔖️Diff
