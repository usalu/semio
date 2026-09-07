//! ➕️ Energy model mutation — `AddConstructionLayer`: Inserts one material layer into a construction at a stated position, outside-to-inside. Layer order is physically load-bearing — the same layers in a different order are a different wall — so the position is payload data, not an append.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ➕️ `add-construction-layer` payload. Inserts one material layer into a construction at a stated position, outside-to-inside. Layer order is physically load-bearing — the same layers in a different order are a different wall — so the position is payload data, not an append.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-construction-layer")]
pub struct AddConstructionLayer {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub material_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_construction_layer(id: crate::model::EntityId, index: u32, material_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::AddConstructionLayer(AddConstructionLayer { id, index, material_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddConstructionLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "construction-layer", kind: "add-construction-layer", record: "AddedConstructionLayer" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add layer {} to construction {} at index {}", self.material_id.0, self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
