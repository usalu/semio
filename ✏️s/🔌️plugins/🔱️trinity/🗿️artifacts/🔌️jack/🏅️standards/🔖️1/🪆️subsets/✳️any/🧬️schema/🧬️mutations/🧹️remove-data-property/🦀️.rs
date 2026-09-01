//! 🧹️ TrinityGraph mutation — `RemoveDataProperty`: takes one key out of a node's or edge's
//! property bag (addressed via `EntityRef`).
use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Mutation
/// 🧹️ `remove-data-property` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveDataProperty {
    pub entity: EntityRef,
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_data_property(entity: EntityRef, key: String) -> TrinityGraphMutation {
    TrinityGraphMutation::RemoveDataProperty(RemoveDataProperty { entity, key })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for RemoveDataProperty {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "data-property", kind: "remove-data-property", record: "RemovedDataProperty" };

    fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        let (kind, id) = match &self.entity {
            EntityRef::Node(id) => ("node", id),
            EntityRef::Edge(id) => ("edge", id),
        };
        format!("Remove {kind} \"{id}\" property \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        match &self.entity {
            EntityRef::Node(id) | EntityRef::Edge(id) => vec![id.clone()],
        }
    }
}
//#endregion 🔖️Mutation
