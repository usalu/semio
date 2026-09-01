//! 🔺️ Diff for `CreateNode`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphNode, SemioGraphPort, SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id.value), [payload.id.value.clone()]);
    }
    let mut nodes = base.nodes.clone();
    nodes.push(SemioGraphNode { id: payload.id.clone(), kind: payload.kind.clone(), label: payload.label.clone(), position: payload.position.clone(), ports: payload.ports.clone(), properties: payload.properties.clone() });
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
