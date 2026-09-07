//! 🔴️ Energy model mutation — `ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp`: Sets the warmest air the ideal-loads system may deliver, in °C.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` payload. Sets the warmest air the ideal-loads system may deliver, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-heating-supply-air-temp")]
pub struct ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp {
    pub id: crate::model::EntityId,
    pub new_max_heating_supply_air_temp_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ideal_loads_system_max_heating_supply_air_temp(id: crate::model::EntityId, new_max_heating_supply_air_temp_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp(ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp { id, new_max_heating_supply_air_temp_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-max-heating-supply-air-temp", record: "ChangedIdealLoadsSystemMaxHeatingSupplyAirTemp" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ideal loads system {} maximum heating supply air temperature to {:?}", self.id.0, self.new_max_heating_supply_air_temp_c)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
