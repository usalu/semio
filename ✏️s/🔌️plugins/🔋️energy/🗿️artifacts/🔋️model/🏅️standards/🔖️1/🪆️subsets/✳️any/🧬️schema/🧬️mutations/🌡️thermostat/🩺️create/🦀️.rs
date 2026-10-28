//! 🩺️ Energy model mutation — `CreateThermostat`: Creates the setpoint control of one zone. The two setpoint schedules must already be defined by the model's own `ScheduleSet` and the zone must exist, so a thermostat can never be born dangling.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🩺️ `create-thermostat` payload. Creates the setpoint control of one zone. The two setpoint schedules must already be defined by the model's own `ScheduleSet` and the zone must exist, so a thermostat can never be born dangling.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-thermostat")]
pub struct CreateThermostat {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub heating_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub cooling_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_thermostat<Op: From<CreateThermostat>>(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    heating_setpoint_schedule_id: crate::schedule::ScheduleId,
    cooling_setpoint_schedule_id: crate::schedule::ScheduleId,
    heating_throttle_range_k: f64,
    cooling_throttle_range_k: f64,
    index: Option<u32>,
) -> Op {
    CreateThermostat { id, zone_id, heating_setpoint_schedule_id, cooling_setpoint_schedule_id, heating_throttle_range_k, cooling_throttle_range_k, index }.into()
}
//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
