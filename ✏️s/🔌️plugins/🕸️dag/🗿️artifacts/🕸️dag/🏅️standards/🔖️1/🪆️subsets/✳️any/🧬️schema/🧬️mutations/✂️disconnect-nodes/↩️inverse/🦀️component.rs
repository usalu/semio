//! ↩️ Inverse for `DisconnectNodes` — reconstructs a `connect-nodes` at the exact captured edge
//! BASE showed (id, endpoints, route style, properties). Missing target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DisconnectNodes, base: &DagSnapshot) -> Vec<DagMutation> {
    match base.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![crate::artifacts::dag::mutations::connect_nodes::mutation::connect_nodes(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
