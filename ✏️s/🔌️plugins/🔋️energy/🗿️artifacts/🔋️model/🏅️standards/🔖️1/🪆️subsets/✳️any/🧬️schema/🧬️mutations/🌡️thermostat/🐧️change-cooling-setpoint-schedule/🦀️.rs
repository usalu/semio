//! 🐧️ Energy model mutation — `ChangeThermostatCoolingSetpointSchedule`: Repoints the cooling setpoint at another schedule the model defines.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐧️ `change-thermostat-cooling-setpoint-schedule` payload. Repoints the cooling setpoint at another schedule the model defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
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
//#endregion 🔖️Mutation
