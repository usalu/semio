//! ☕️ Energy model mutation — `ChangePlantLoopSupplyTemperature`: Sets the temperature the loop supplies at, in °C.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☕️ `change-plant-loop-supply-temperature` payload. Sets the temperature the loop supplies at, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-supply-temperature")]
pub struct ChangePlantLoopSupplyTemperature {
    pub id: crate::model::EntityId,
    pub new_supply_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_plant_loop_supply_temperature(id: crate::model::EntityId, new_supply_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePlantLoopSupplyTemperature(ChangePlantLoopSupplyTemperature { id, new_supply_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePlantLoopSupplyTemperature {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "plant-loop", kind: "change-plant-loop-supply-temperature", record: "ChangedPlantLoopSupplyTemperature" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change plant loop {} supply temperature to {:?}", self.id.0, self.new_supply_temperature_c)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
