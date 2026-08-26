//! ↩️ Inverse for `DeleteNode` — reconstructs the removed node from BASE, then re-`connect`s every
//! severed edge (in reverse dependency order: node first, edges after). Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::jack::mutations::{create_edge, create_node, TrinityGraphMutation};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteNode, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    let nodes = base.nodes();
    let Some(node) = nodes.iter().find(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut out = vec![create_node(node.clone())];
    for edge in base.edges().iter().filter(|edge| crate::artifacts::jack::port_node_id(&edge.source) == Some(payload.id.as_str()) || crate::artifacts::jack::port_node_id(&edge.target) == Some(payload.id.as_str())) {
        out.push(create_edge(edge.clone()));
    }
    out
}
//#endregion 🔖️Inverse
