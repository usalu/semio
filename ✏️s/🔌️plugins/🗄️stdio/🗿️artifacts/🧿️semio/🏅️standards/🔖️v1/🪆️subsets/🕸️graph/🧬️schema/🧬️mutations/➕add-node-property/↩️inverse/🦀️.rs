//! ↩️ Inverse for `AddNodeProperty`.

use crate::standards::v1::subsets::graph::schema::mutations::{remove_node_property::RemoveNodeProperty, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `remove-node-property` of the attached key; nothing when the add would not apply (no node, or
/// the key already present — undoing that no-op must never detach the pre-existing entry).
pub fn inverse(payload: &super::AddNodeProperty, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok(base
        .nodes
        .iter()
        .find(|node| node.id == payload.node_id && !node.properties.iter().any(|property| property.key == payload.property.key))
        .map(|_| SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: payload.node_id.clone(), key: payload.property.key.clone() }))
        .into_iter()
        .collect())
}
//#endregion 🔖️Inverse
