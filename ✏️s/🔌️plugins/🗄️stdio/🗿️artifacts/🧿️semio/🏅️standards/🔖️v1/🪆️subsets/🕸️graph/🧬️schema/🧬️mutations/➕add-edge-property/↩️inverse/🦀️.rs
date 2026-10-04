//! ↩️ Inverse for `AddEdgeProperty`.

use crate::standards::v1::subsets::graph::schema::mutations::{remove_edge_property::RemoveEdgeProperty, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `remove-edge-property` of the attached key; nothing when the add would not apply.
pub fn inverse(payload: &super::AddEdgeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    base.edges
        .iter()
        .find(|edge| edge.id == payload.edge_id && !payload.property.key.is_empty() && !edge.properties.iter().any(|property| property.key == payload.property.key))
        .map(|_| SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: payload.edge_id.clone(), key: payload.property.key.clone() }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
