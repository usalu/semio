//! 🔺️ `create-node` — sparse diff construction; if a node with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphNode, SemioGraphSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if !nodes.iter().any(|n| n.id == payload.id) {
        nodes.push(SemioGraphNode { id: payload.id.clone(), kind: payload.kind.clone(), label: payload.label.clone(), position: payload.position.clone(), ports: payload.ports.clone(), properties: payload.properties.clone() });
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
