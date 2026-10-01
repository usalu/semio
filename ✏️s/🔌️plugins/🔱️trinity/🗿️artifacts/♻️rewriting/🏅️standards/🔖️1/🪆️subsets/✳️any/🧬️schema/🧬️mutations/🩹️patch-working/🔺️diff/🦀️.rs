//! 🔺️ Sparse diff builder for `PatchWorkingNodes` — every addressed node of the working graph gets the field's value, the graph
//! written back as compact JSON with sorted keys (`before_fixture_json`) once its manifest still declares every kind.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::PatchWorkingNodes, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a patch names at least one node, never one twice, the name or kind field and a non-blank value", payload.targets.clone());
    }
    let value = payload.value.trim().to_string();
    let patched = super::super::edit_working_graph_nodes(&base.before_fixture_json, &payload.targets, |node| {
        let next = serde_json::Value::String(value.clone());
        let changed = node.get(&payload.field) != Some(&next);
        node.insert(payload.field.clone(), next);
        changed
    });
    let Some((json, missing, changed)) = patched else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it has none of the targets", payload.targets.clone());
    };
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a node of the working graph", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (not in the working graph): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if !changed {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", format!("every target's {} is already “{value}”", payload.field)).at(payload.targets.clone())]));
    }
    if payload.field == "kind" && !super::super::working_graph_is_valid(&json) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("the working graph's manifest declares no node kind “{value}”"), payload.targets.clone());
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
