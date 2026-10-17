//! ↩️ Inverse for `RemoveNodeProperty`.

use crate::schema::mutations::add_node_property::AddNodeProperty;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `add-node-property` of the removed entry at its BASE index (byte-exact); nothing when the key is absent.
pub fn inverse<Op>(payload: &super::RemoveNodeProperty, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::add_node_property::AddNodeProperty>,
{
    base.nodes
        .iter()
        .find(|node| node.id == payload.node_id)
        .and_then(|node| node.properties.iter().position(|property| property.key == payload.key).map(|index| (index, node.properties[index].clone())))
        .map(|(index, property)| Op::from(AddNodeProperty { node_id: payload.node_id.clone(), index, property }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
