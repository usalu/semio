//! 🎚️ Energy model mutation — `ChangeThermostatHeatingThrottleRange`: Sets the proportional band the heating setpoint is approached over, in kelvin.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎚️ `change-thermostat-heating-throttle-range` payload. Sets the proportional band the heating setpoint is approached over, in kelvin.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-heating-throttle-range")]
pub struct ChangeThermostatHeatingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_heating_throttle_range_k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_thermostat_heating_throttle_range(id: crate::model::EntityId, new_heating_throttle_range_k: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeThermostatHeatingThrottleRange(ChangeThermostatHeatingThrottleRange { id, new_heating_throttle_range_k })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeThermostatHeatingThrottleRange {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-heating-throttle-range", record: "ChangedThermostatHeatingThrottleRange" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change thermostat {} heating throttle range to {:?}", self.id.0, self.new_heating_throttle_range_k)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
