//! 🐧️ Energy model mutation — `ChangeThermostatCoolingSetpointSchedule`: Repoints the cooling setpoint at another schedule the model defines.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐧️ `change-thermostat-cooling-setpoint-schedule` payload. Repoints the cooling setpoint at another schedule the model defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-cooling-setpoint-schedule")]
pub struct ChangeThermostatCoolingSetpointSchedule {
    pub id: crate::model::EntityId,
    pub new_cooling_setpoint_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_thermostat_cooling_setpoint_schedule<Op: From<ChangeThermostatCoolingSetpointSchedule>>(id: crate::model::EntityId, new_cooling_setpoint_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeThermostatCoolingSetpointSchedule { id, new_cooling_setpoint_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeThermostatCoolingSetpointSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeThermostatCoolingSetpointSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-cooling-setpoint-schedule", record: "ChangedThermostatCoolingSetpointSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change thermostat {} cooling setpoint schedule to {}", self.id.0, self.new_cooling_setpoint_schedule_id.0), &format!("Kühl-Sollwertzeitplan von Thermostat {} auf {} ändern", self.id.0, self.new_cooling_setpoint_schedule_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeThermostatCoolingSetpointSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeThermostatCoolingSetpointSchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
