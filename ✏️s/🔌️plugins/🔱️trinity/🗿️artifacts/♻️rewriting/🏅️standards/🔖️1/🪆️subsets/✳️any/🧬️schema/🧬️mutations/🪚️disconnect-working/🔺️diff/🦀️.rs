//! 🔺️ Sparse diff builder for `DisconnectWorkingEdges` — every addressed edge of the working graph removed, the graph written back
//! as compact JSON with sorted keys (`before_fixture_json`).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectWorkingEdges, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a cut names at least one edge, never one twice", payload.targets.clone());
    }
    let Some((json, missing)) = super::super::remove_working_graph_edges(&base.before_fixture_json, &payload.targets) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it has none of the targets", payload.targets.clone());
    };
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an edge of the working graph", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (not in the working graph): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
