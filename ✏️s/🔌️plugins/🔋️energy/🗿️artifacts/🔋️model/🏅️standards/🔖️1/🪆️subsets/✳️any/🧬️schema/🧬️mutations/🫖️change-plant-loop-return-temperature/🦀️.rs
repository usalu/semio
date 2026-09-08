//! 🧫️ Energy model mutation — `ChangePlantLoopReturnTemperature`: Sets the temperature the loop returns at, in °C.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧫️ `change-plant-loop-return-temperature` payload. Sets the temperature the loop returns at, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-return-temperature")]
pub struct ChangePlantLoopReturnTemperature {
    pub id: crate::model::EntityId,
    pub new_return_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_plant_loop_return_temperature(id: crate::model::EntityId, new_return_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePlantLoopReturnTemperature(ChangePlantLoopReturnTemperature { id, new_return_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePlantLoopReturnTemperature {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "plant-loop", kind: "change-plant-loop-return-temperature", record: "ChangedPlantLoopReturnTemperature" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change plant loop {} return temperature to {:?}", self.id.0, self.new_return_temperature_c)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
