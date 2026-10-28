//! 🌂️ Energy model mutation — `CreateHumidistat`: Creates the humidity control of one zone, against two setpoint schedules the model already defines.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌂️ `create-humidistat` payload. Creates the humidity control of one zone, against two setpoint schedules the model already defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-humidistat")]
pub struct CreateHumidistat {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub humidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub dehumidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub humidifying_throttle_range: f64,
    pub dehumidifying_throttle_range: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_humidistat<Op: From<CreateHumidistat>>(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    humidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    dehumidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    humidifying_throttle_range: f64,
    dehumidifying_throttle_range: f64,
    index: Option<u32>,
) -> Op {
    CreateHumidistat { id, zone_id, humidifying_setpoint_schedule_id, dehumidifying_setpoint_schedule_id, humidifying_throttle_range, dehumidifying_throttle_range, index }.into()
}
//#endregion 🔖️Mutation
