//! ↩️ Inverse for `RemoveNodeProperty`.

use crate::standards::v1::subsets::graph::schema::mutations::{add_node_property::AddNodeProperty, SemioGraphMutation};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `add-node-property` of the removed entry at its BASE index (byte-exact); nothing when the key is absent.
pub fn inverse(payload: &super::RemoveNodeProperty, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    base.nodes
        .iter()
        .find(|node| node.id == payload.node_id)
        .and_then(|node| node.properties.iter().position(|property| property.key == payload.key).map(|index| (index, node.properties[index].clone())))
        .map(|(index, property)| SemioGraphMutation::AddNodeProperty(AddNodeProperty { node_id: payload.node_id.clone(), index, property }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
