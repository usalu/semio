//! ☔️ Energy model mutation — `ChangeHumidistatHumidifyingSetpointSchedule`: Repoints the humidifying setpoint at another schedule the model defines.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☔️ `change-humidistat-humidifying-setpoint-schedule` payload. Repoints the humidifying setpoint at another schedule the model defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-humidifying-setpoint-schedule")]
pub struct ChangeHumidistatHumidifyingSetpointSchedule {
    pub id: crate::model::EntityId,
    pub new_humidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_humidistat_humidifying_setpoint_schedule<Op: From<ChangeHumidistatHumidifyingSetpointSchedule>>(id: crate::model::EntityId, new_humidifying_setpoint_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeHumidistatHumidifyingSetpointSchedule { id, new_humidifying_setpoint_schedule_id }.into()
}
//#endregion 🔖️Mutation
