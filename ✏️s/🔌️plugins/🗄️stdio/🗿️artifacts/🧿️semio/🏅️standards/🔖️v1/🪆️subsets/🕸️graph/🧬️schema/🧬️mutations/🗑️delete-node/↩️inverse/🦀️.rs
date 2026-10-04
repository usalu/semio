//! ↩️ Inverse for `DeleteNode`.

use crate::standards::v1::subsets::graph::schema::mutations::{create_edge::CreateEdge, create_node::CreateNode, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `create-node` of the whole node record at its BASE index, then one `create-edge` per severed edge at
/// its BASE index in ascending order, so undo restores the before-snapshot byte for byte (and its content address); nothing
/// when the node is absent.
pub fn inverse(payload: &super::DeleteNode, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    let Some(index) = base.nodes.iter().position(|node| node.id == payload.id) else { return Ok(Vec::new()) };
    let node = &base.nodes[index];
    let restore = SemioGraphMutation::CreateNode(CreateNode { id: node.id.clone(), kind: node.kind.clone(), label: node.label.clone(), position: node.position, width: node.width, height: node.height, ports: node.ports.clone(), properties: node.properties.clone(), at: Some(index) });
    let edges = base.edges.iter().enumerate().filter(|(_, edge)| edge.source == payload.id || edge.target == payload.id).map(|(at, edge)| {
        SemioGraphMutation::CreateEdge(CreateEdge { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), kind: edge.kind.clone(), label: edge.label.clone(), source_port: edge.source_port.clone(), target_port: edge.target_port.clone(), properties: edge.properties.clone(), at: Some(at) })
    });
    Ok(std::iter::once(restore).chain(edges).collect())
}
//#endregion 🔖️Inverse
