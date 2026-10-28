//! 🥵️ Energy model mutation — `ChangeThermostatHeatingSetpointSchedule`: Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥵️ `change-thermostat-heating-setpoint-schedule` payload. Repoints the heating setpoint at another schedule the model defines. Setpoint schedules carry °C directly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
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
//#endregion 🔖️Mutation
