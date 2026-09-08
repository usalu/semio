//! 🥵️ Energy model mutation — `ChangeThermostatHeatingSetpointSchedule`: Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥵️ `change-thermostat-heating-setpoint-schedule` payload. Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-heating-setpoint-schedule")]
pub struct ChangeThermostatHeatingSetpointSchedule {
    pub id: crate::model::EntityId,
    pub new_heating_setpoint_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_thermostat_heating_setpoint_schedule(id: crate::model::EntityId, new_heating_setpoint_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeThermostatHeatingSetpointSchedule(ChangeThermostatHeatingSetpointSchedule { id, new_heating_setpoint_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeThermostatHeatingSetpointSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-heating-setpoint-schedule", record: "ChangedThermostatHeatingSetpointSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change thermostat {} heating setpoint schedule to {}", self.id.0, self.new_heating_setpoint_schedule_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
