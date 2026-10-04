//! ↩️ Inverse for `RemoveEdgeProperty`.

use crate::standards::v1::subsets::graph::schema::mutations::{add_edge_property::AddEdgeProperty, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `add-edge-property` of the removed entry at its BASE index (byte-exact); nothing when the key is absent.
pub fn inverse(payload: &super::RemoveEdgeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    base.edges
        .iter()
        .find(|edge| edge.id == payload.edge_id)
        .and_then(|edge| edge.properties.iter().position(|property| property.key == payload.key).map(|index| (index, edge.properties[index].clone())))
        .map(|(index, property)| SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: payload.edge_id.clone(), index, property }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
