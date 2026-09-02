//! ↩️ Inverse for `RemoveDataProperty` — the OLD value looked up from BASE, restored via
//! `change-data-property`. Missing target/key ⇒ `Vec::new()`.
use crate::artifacts::jack::mutations::{change_data_property, TrinityGraphMutation};
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Inverse
fn base_property_value(base: &JackSnapshot, entity: &EntityRef, key: &str) -> Option<crate::artifacts::jack::PropertyValue> {
    match entity {
        EntityRef::Node(id) => base.nodes().iter().find(|node| node.id == *id).and_then(|node| node.properties.get(key).cloned()),
        EntityRef::Edge(id) => base.edges().iter().find(|edge| edge.id == *id).and_then(|edge| edge.properties.get(key).cloned()),
    }
}

pub fn inverse(payload: &super::RemoveDataProperty, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    base_property_value(base, &payload.entity, &payload.key).map(|old| vec![change_data_property(payload.entity.clone(), payload.key.clone(), old)]).unwrap_or_default()
}
//#endregion 🔖️Inverse
