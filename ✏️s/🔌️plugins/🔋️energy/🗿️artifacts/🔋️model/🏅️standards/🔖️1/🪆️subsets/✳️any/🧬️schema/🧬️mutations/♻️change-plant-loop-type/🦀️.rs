//! ♻️ Energy model mutation — `ChangePlantLoopType`: Swaps which fluid duty the loop carries.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♻️ `change-plant-loop-type` payload. Swaps which fluid duty the loop carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-type")]
pub struct ChangePlantLoopType {
    pub id: crate::model::EntityId,
    pub new_loop_type: crate::model::PlantLoopType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_plant_loop_type(id: crate::model::EntityId, new_loop_type: crate::model::PlantLoopType) -> EnergyModelMutation {
    EnergyModelMutation::ChangePlantLoopType(ChangePlantLoopType { id, new_loop_type })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePlantLoopType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "plant-loop", kind: "change-plant-loop-type", record: "ChangedPlantLoopType" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change plant loop {} loop type to {:?}", self.id.0, self.new_loop_type)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
