//! 🔺️ Diff for `DeleteNode`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if !base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    }
    let mut nodes = base.nodes.clone();
    nodes.retain(|n| n.id != payload.id);
    let mut edges = base.edges.clone();
    let severed = edges.iter().filter(|e| e.source == payload.id || e.target == payload.id).count();
    edges.retain(|e| e.source != payload.id && e.target != payload.id);
    let outcome = protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: Some(SemioGraphEdgeList { values: edges }) });
    if severed > 0 {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also severed {severed} edge(s).", payload.id.value))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
