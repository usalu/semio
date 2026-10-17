//! ↩️ Inverse for `RemoveEdgeProperty`.

use crate::schema::mutations::add_edge_property::AddEdgeProperty;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `add-edge-property` of the removed entry at its BASE index (byte-exact); nothing when the key is absent.
pub fn inverse<Op>(payload: &super::RemoveEdgeProperty, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::add_edge_property::AddEdgeProperty>,
{
    base.edges
        .iter()
        .find(|edge| edge.id == payload.edge_id)
        .and_then(|edge| edge.properties.iter().position(|property| property.key == payload.key).map(|index| (index, edge.properties[index].clone())))
        .map(|(index, property)| Op::from(AddEdgeProperty { edge_id: payload.edge_id.clone(), index, property }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
