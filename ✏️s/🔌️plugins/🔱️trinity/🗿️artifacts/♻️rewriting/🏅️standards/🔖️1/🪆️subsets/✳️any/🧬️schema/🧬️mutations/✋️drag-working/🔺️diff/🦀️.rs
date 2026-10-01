//! 🔺️ Sparse diff builder for `DragWorkingNodes` — every addressed node of the working graph moved by the offset read off its
//! BASE position, the graph written back as compact JSON with sorted keys (`before_fixture_json`).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DragWorkingNodes, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag names at least one node, never one twice, by a finite offset", payload.targets.clone());
    }
    let moved = super::super::edit_working_graph_nodes(&base.before_fixture_json, &payload.targets, |node| {
        for (axis, offset) in [("x", payload.dx), ("y", payload.dy)] {
            let at = node.get(axis).and_then(serde_json::Value::as_f64).unwrap_or(0.0);
            node.insert(axis.to_string(), serde_json::Value::from(at + offset));
        }
        true
    });
    let Some((json, missing, _)) = moved else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it has none of the targets", payload.targets.clone());
    };
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a node of the working graph", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (not in the working graph): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "a zero offset moves nothing").at(payload.targets.clone())]));
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
