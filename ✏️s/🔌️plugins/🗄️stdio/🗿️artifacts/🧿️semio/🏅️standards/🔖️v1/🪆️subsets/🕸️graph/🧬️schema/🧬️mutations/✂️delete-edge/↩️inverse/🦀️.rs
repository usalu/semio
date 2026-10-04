//! ↩️ Inverse for `DeleteEdge`.

use crate::standards::v1::subsets::graph::schema::mutations::{create_edge::CreateEdge, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `create-edge` of the whole edge record at its BASE index; nothing when the edge is absent.
pub fn inverse(payload: &super::DeleteEdge, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok(base
        .edges
        .iter()
        .position(|edge| edge.id == payload.id)
        .map(|at| {
            let edge = &base.edges[at];
            SemioGraphMutation::CreateEdge(CreateEdge { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), kind: edge.kind.clone(), label: edge.label.clone(), source_port: edge.source_port.clone(), target_port: edge.target_port.clone(), properties: edge.properties.clone(), at: Some(at) })
        })
        .into_iter()
        .collect())
}
//#endregion 🔖️Inverse
