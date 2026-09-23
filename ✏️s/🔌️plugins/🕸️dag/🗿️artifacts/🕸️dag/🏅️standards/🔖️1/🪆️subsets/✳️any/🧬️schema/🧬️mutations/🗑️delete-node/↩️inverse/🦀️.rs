//! ↩️ Inverse for `DeleteNode` — re-creates the captured BASE node at its exact BASE position, then
//! reconnects every severed edge at its exact BASE position, ascending, so the content child (a hash
//! over node and edge order) is restored byte-for-byte. A node with no incident edge inverts to ONE
//! row, which is what a retained one-item preparation (`for_one_invertible_item`) can fold; the
//! editor's removal verbs disconnect incident edges first so every row they publish stays
//! point-invertible (`remove_nodes_operations`). Missing target ⇒ `Vec::new()`.
use crate::mutations::DagMutation;
use crate::schema::split_endpoint;
use crate::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> Vec<DagMutation> {
    let scene = dag_working_scene(base);
    let Some(position) = scene.nodes.iter().position(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut mutations = vec![crate::mutations::create_node::mutation::create_node_at(scene.nodes[position].clone(), position)];
    for (index, edge) in scene.edges.iter().enumerate().filter(|(_, edge)| split_endpoint(&edge.source).0 == payload.id || split_endpoint(&edge.target).0 == payload.id) {
        mutations.push(crate::mutations::connect_nodes::mutation::connect_nodes_at(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone(), index));
    }
    mutations.reverse();
    mutations
}
//#endregion 🔖️Inverse
