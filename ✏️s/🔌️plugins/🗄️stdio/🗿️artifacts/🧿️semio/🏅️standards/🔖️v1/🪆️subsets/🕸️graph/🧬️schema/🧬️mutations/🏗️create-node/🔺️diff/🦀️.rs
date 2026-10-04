//! 🔺️ Diff for `CreateNode`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphNode, SemioGraphSnapshot};

//#region 🔖️Diff
/// 🧮️ A node id the graph already carries is a Fatal `mutation.duplicate-id`; otherwise the node is inserted at `at`
/// (clamped to the end) or appended when `at` is absent — `delete-node`'s exact undo names the removed node's index.
pub fn diff(payload: &super::CreateNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id.value), [payload.id.value.clone()]);
    }
    let mut nodes = base.nodes.clone();
    nodes.insert(payload.at.map_or(nodes.len(), |at| at.min(nodes.len())), SemioGraphNode { id: payload.id.clone(), kind: payload.kind.clone(), label: payload.label.clone(), position: payload.position, width:payload.width,height:payload.height, ports: payload.ports.clone(), properties: payload.properties.clone() });
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
