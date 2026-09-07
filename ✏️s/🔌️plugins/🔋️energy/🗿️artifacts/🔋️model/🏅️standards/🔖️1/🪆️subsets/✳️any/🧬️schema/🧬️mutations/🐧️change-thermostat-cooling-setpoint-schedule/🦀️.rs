//! 🐧️ Energy model mutation — `ChangeThermostatCoolingSetpointSchedule`: Repoints the cooling setpoint at another schedule the model defines.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐧️ `change-thermostat-cooling-setpoint-schedule` payload. Repoints the cooling setpoint at another schedule the model defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-cooling-setpoint-schedule")]
pub struct ChangeThermostatCoolingSetpointSchedule {
    pub id: crate::model::EntityId,
    pub new_cooling_setpoint_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_thermostat_cooling_setpoint_schedule(id: crate::model::EntityId, new_cooling_setpoint_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeThermostatCoolingSetpointSchedule(ChangeThermostatCoolingSetpointSchedule { id, new_cooling_setpoint_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeThermostatCoolingSetpointSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-cooling-setpoint-schedule", record: "ChangedThermostatCoolingSetpointSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change thermostat {} cooling setpoint schedule to {}", self.id.0, self.new_cooling_setpoint_schedule_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
