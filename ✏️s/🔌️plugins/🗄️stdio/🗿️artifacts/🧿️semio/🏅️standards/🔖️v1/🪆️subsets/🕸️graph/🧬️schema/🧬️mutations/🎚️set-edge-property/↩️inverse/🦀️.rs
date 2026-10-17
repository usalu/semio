//! ↩️ Inverse for `SetEdgeProperty`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `set-edge-property` back to the BASE value of that key; nothing when the edge or key is absent.
pub fn inverse<Op>(payload: &super::SetEdgeProperty, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::set_edge_property::SetEdgeProperty>,
{
    base.edges
        .iter()
        .filter(|edge| edge.id == payload.edge_id)
        .flat_map(|edge| edge.properties.iter())
        .find(|property| property.key == payload.key && property.value != payload.value)
        .map(|property| Op::from(super::SetEdgeProperty { edge_id: payload.edge_id.clone(), key: payload.key.clone(), value: property.value.clone() }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
