//! 🔺️ `create-edge` — sparse diff construction; Fatal `mutation.duplicate-id` if an edge with this
//! `id` already exists in `base` (real entity-lifecycle safety — never a duplicate id), Fatal
//! `mutation.invariant` if `source`/`target` reference a node id absent from `base.nodes`.

use super::mutation::CreateEdge;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateEdge, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if base.edges.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id.value), [payload.id.value.clone()]).await;
    }
    if !base.nodes.iter().any(|n| n.id == payload.source) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" references unknown source node \"{}\".", payload.id.value, payload.source.value), [payload.id.value.clone(), payload.source.value.clone()]).await;
    }
    if !base.nodes.iter().any(|n| n.id == payload.target) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" references unknown target node \"{}\".", payload.id.value, payload.target.value), [payload.id.value.clone(), payload.target.value.clone()]).await;
    }
    let mut edges = base.edges.clone();
    edges.push(SemioGraphEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), kind: payload.kind.clone(), label: payload.label.clone() });
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) }).await
}
//#endregion 🔖️Diff
