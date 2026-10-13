//! ↩️ Inverse for `SetNodeProperty`.

use crate::schema::mutations::SemioGraphMutation;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `set-node-property` back to the BASE value of that key; nothing when the node or key is absent.
pub fn inverse(payload: &super::SetNodeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    base.nodes
        .iter()
        .filter(|node| node.id == payload.node_id)
        .flat_map(|node| node.properties.iter())
        .find(|property| property.key == payload.key && property.value != payload.value)
        .map(|property| SemioGraphMutation::SetNodeProperty(super::SetNodeProperty { node_id: payload.node_id.clone(), key: payload.key.clone(), value: property.value.clone() }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
