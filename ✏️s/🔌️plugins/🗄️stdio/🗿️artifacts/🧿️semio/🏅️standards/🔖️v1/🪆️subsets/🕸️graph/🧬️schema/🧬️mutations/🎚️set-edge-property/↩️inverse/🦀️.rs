//! ↩️ Inverse for `SetEdgeProperty`.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `set-edge-property` back to the BASE value of that key; nothing when the edge or key is absent.
pub fn inverse(payload: &super::SetEdgeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    base.edges
        .iter()
        .filter(|edge| edge.id == payload.edge_id)
        .flat_map(|edge| edge.properties.iter())
        .find(|property| property.key == payload.key && property.value != payload.value)
        .map(|property| SemioGraphMutation::SetEdgeProperty(super::SetEdgeProperty { edge_id: payload.edge_id.clone(), key: payload.key.clone(), value: property.value.clone() }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
