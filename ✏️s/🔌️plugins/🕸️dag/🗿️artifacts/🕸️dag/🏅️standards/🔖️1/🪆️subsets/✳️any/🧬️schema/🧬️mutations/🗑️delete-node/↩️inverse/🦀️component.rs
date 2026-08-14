//! ↩️ Inverse for `DeleteNode` — reconstructs the captured BASE node and the exact BASE node/edge
//! order through typed mutations. Missing target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> Vec<DagMutation> {
    let scene = dag_working_scene(base);
    let Some(node) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut mutations = vec![
        crate::artifacts::dag::mutations::create_node::mutation::create_node(node.clone()),
        crate::artifacts::dag::mutations::reorder_nodes::mutation::reorder_nodes(scene.nodes.iter().map(|node| node.id.clone()).collect()),
    ];
    for edge in &scene.edges {
        mutations.push(crate::artifacts::dag::mutations::disconnect_nodes::mutation::disconnect_nodes(edge.id.clone()));
    }
    for edge in &scene.edges {
        mutations.push(crate::artifacts::dag::mutations::connect_nodes::mutation::connect_nodes(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone()));
    }
    mutations.reverse();
    mutations
}
//#endregion 🔖️Inverse
