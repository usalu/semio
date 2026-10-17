//! ↩️ Inverse for `SetNodeProperty`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `set-node-property` back to the BASE value of that key; nothing when the node or key is absent.
pub fn inverse<Op>(payload: &super::SetNodeProperty, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::set_node_property::SetNodeProperty>,
{
    base.nodes
        .iter()
        .filter(|node| node.id == payload.node_id)
        .flat_map(|node| node.properties.iter())
        .find(|property| property.key == payload.key && property.value != payload.value)
        .map(|property| Op::from(super::SetNodeProperty { node_id: payload.node_id.clone(), key: payload.key.clone(), value: property.value.clone() }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
