//! 🔧️ TrinityGraph mutation — `ChangeDataProperty`: upserts one key on a node's or edge's property
//! bag (addressed via `EntityRef`).
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{EntityRef, JackSnapshot, PropertyValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔧️ `change-data-property` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDataProperty {
    pub entity: EntityRef,
    pub key: String,
    pub new_value: PropertyValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_data_property(entity: EntityRef, key: String, new_value: PropertyValue) -> TrinityGraphMutation {
    TrinityGraphMutation::ChangeDataProperty(ChangeDataProperty { entity, key, new_value })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for ChangeDataProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "data-property", kind: "change-data-property", record: "ChangedDataProperty" };

    async fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        let (kind, id) = match &self.entity {
            EntityRef::Node(id) => ("node", id),
            EntityRef::Edge(id) => ("edge", id),
        };
        format!("Change {kind} \"{id}\" property \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        match &self.entity {
            EntityRef::Node(id) | EntityRef::Edge(id) => vec![id.clone()],
        }
    }
}
//#endregion 🔖️Mutation
