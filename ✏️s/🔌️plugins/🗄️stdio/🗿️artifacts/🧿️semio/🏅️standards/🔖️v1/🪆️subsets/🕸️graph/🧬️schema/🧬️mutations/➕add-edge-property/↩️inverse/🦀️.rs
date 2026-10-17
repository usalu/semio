//! ↩️ Inverse for `AddEdgeProperty`.

use crate::schema::mutations::remove_edge_property::RemoveEdgeProperty;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `remove-edge-property` of the attached key; nothing when the add would not apply.
pub fn inverse<Op>(payload: &super::AddEdgeProperty, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::remove_edge_property::RemoveEdgeProperty>,
{
    base.edges
        .iter()
        .find(|edge| edge.id == payload.edge_id && !payload.property.key.is_empty() && !edge.properties.iter().any(|property| property.key == payload.property.key))
        .map(|_| Op::from(RemoveEdgeProperty { edge_id: payload.edge_id.clone(), key: payload.property.key.clone() }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
