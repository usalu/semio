//! 🔺️ `create-node` — sparse diff construction; Fatal `mutation.duplicate-id` if a node with this
//! `id` already exists in `base` (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphNode, SemioGraphSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id.value), [payload.id.value.clone()]);
    }
    let mut nodes = base.nodes.clone();
    nodes.push(SemioGraphNode { id: payload.id.clone(), kind: payload.kind.clone(), label: payload.label.clone(), position: payload.position.clone(), ports: payload.ports.clone(), properties: payload.properties.clone() });
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
