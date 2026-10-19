//! 🥵️ Energy model mutation — `ChangeThermostatHeatingSetpointSchedule`: Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥵️ `change-thermostat-heating-setpoint-schedule` payload. Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-heating-setpoint-schedule")]
pub struct ChangeThermostatHeatingSetpointSchedule {
    pub id: crate::model::EntityId,
    pub new_heating_setpoint_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_thermostat_heating_setpoint_schedule<Op: From<ChangeThermostatHeatingSetpointSchedule>>(id: crate::model::EntityId, new_heating_setpoint_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeThermostatHeatingSetpointSchedule { id, new_heating_setpoint_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeThermostatHeatingSetpointSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeThermostatHeatingSetpointSchedule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change thermostat {} heating setpoint schedule to {}", self.id.0, self.new_heating_setpoint_schedule_id.0), &format!("Heiz-Sollwertzeitplan von Thermostat {} auf {} ändern", self.id.0, self.new_heating_setpoint_schedule_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeThermostatHeatingSetpointSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeThermostatHeatingSetpointSchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-heating-setpoint-schedule", record: "ChangedThermostatHeatingSetpointSchedule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
