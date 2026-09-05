//! ↩️ Inverse for `DeleteNode`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, create_edge, create_node};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteNode, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return Vec::new();
    };
    let mut out = vec![SemioGraphMutation::CreateNode(create_node::CreateNode {
        id: node.id.clone(),
        kind: node.kind.clone(),
        label: node.label.clone(),
        position: node.position.clone(),
        ports: node.ports.clone(),
        properties: node.properties.clone(),
    })];
    for edge in base.edges.iter().filter(|e| e.source == payload.id || e.target == payload.id) {
        out.push(SemioGraphMutation::CreateEdge(create_edge::CreateEdge { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), kind: edge.kind.clone(), label: edge.label.clone() }));
    }
    out
}
//#endregion 🔖️Inverse
