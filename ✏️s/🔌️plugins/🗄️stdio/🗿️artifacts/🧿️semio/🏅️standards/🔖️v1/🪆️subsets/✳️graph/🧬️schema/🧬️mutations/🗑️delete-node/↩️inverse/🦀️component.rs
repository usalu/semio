//! ↩️ `delete-node` — reconstructs the removed node from BASE, then re-`create`s every edge
//! severed by its removal (in reverse dependency order: node first, edges after — the same idea
//! `📌️important.md`'s "re-connecting severed links after create" describes, adapted to this
//! entity model's `create`/`delete` verbs). Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{create_edge, create_node, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteNode, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return Vec::new();
    };
    let mut out = vec![SemioGraphMutation::CreateNode(create_node::mutation::CreateNode {
        id: node.id.clone(),
        kind: node.kind.clone(),
        label: node.label.clone(),
        position: node.position.clone(),
        ports: node.ports.clone(),
        properties: node.properties.clone(),
    })];
    for edge in base.edges.iter().filter(|e| e.source == payload.id || e.target == payload.id) {
        out.push(SemioGraphMutation::CreateEdge(create_edge::mutation::CreateEdge {
            id: edge.id.clone(),
            source: edge.source.clone(),
            target: edge.target.clone(),
            kind: edge.kind.clone(),
            label: edge.label.clone(),
        }));
    }
    out
}
//#endregion 🔖️Inverse
