//! ↩️ Inverse for `DeleteNode` — reconstructs a `create-node` of the captured BASE node, then
//! re-`connect-nodes`s every edge BASE shows touching it (severed cascade). Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::schema::split_endpoint;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> Vec<DagMutation> {
    let Some(node) = base.nodes.iter().find(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut mutations = vec![crate::artifacts::dag::mutations::create_node::mutation::create_node(node.clone())];
    for edge in base.edges.iter().filter(|edge| split_endpoint(&edge.source).0 == payload.id || split_endpoint(&edge.target).0 == payload.id) {
        mutations.push(crate::artifacts::dag::mutations::connect_nodes::mutation::connect_nodes(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone()));
    }
    mutations
}
//#endregion 🔖️Inverse
