//! 🕙️ Energy model mutation — `ChangeTimeSeriesScheduleTimestep`: Sets how many seconds one sample of a measured series covers — what turns its index into a clock.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕙️ `change-time-series-schedule-timestep` payload. Sets how many seconds one sample of a measured series covers — what turns its index into a clock.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-time-series-schedule-timestep")]
pub struct ChangeTimeSeriesScheduleTimestep {
    pub id: crate::schedule::ScheduleId,
    pub new_timestep_seconds: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_time_series_schedule_timestep<Op: From<ChangeTimeSeriesScheduleTimestep>>(id: crate::schedule::ScheduleId, new_timestep_seconds: u32) -> Op {
    ChangeTimeSeriesScheduleTimestep { id, new_timestep_seconds }.into()
}
//#endregion 🔖️Mutation
