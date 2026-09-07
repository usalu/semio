//! ➖️ Energy model mutation — `RemoveConstructionLayer`: Removes the material layer a construction holds at a stated position. Addressed by position rather than by material id because the same material may legitimately appear in a construction more than once.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ➖️ `remove-construction-layer` payload. Removes the material layer a construction holds at a stated position. Addressed by position rather than by material id because the same material may legitimately appear in a construction more than once.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-construction-layer")]
pub struct RemoveConstructionLayer {
    pub id: crate::model::EntityId,
    pub index: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_construction_layer(id: crate::model::EntityId, index: u32) -> EnergyModelMutation {
    EnergyModelMutation::RemoveConstructionLayer(RemoveConstructionLayer { id, index })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveConstructionLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "construction-layer", kind: "remove-construction-layer", record: "RemovedConstructionLayer" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove layer {} from construction {}", self.index, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
