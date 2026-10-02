//! 🔺️ Sparse diff builder for `ConnectWorkingPorts` — the wire appended to the working graph's edges once both endpoint nodes are
//! in it, the graph written back as compact JSON with sorted keys (`before_fixture_json`) once its manifest still declares every kind.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectWorkingPorts, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    let endpoints = vec![payload.source.clone(), payload.target.clone()];
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a wire joins two different non-blank endpoints by a non-blank edge kind", endpoints);
    }
    let Some((json, missing, drawn)) = super::super::connect_working_graph_ports(&base.before_fixture_json, &payload.edge_id(), &payload.source, &payload.target, &payload.kind) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it holds neither endpoint", endpoints);
    };
    if !missing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("the working graph holds no node {}", missing.join(", ")), missing);
    }
    if !drawn {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "the working graph already holds this wire").at(endpoints)]);
    }
    if !super::super::working_graph_is_valid(&json) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("the working graph's manifest declares no edge kind “{}”", payload.kind), endpoints);
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() })
}
//#endregion 🔖️Diff
