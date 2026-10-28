//! 🕕️ Energy model mutation — `ChangeDailyScheduleInterpolation`: Sets whether a daily profile steps between its hourly values or ramps across them.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕕️ `change-daily-schedule-interpolation` payload. Sets whether a daily profile steps between its hourly values or ramps across them.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daily-schedule-interpolation")]
pub struct ChangeDailyScheduleInterpolation {
    pub id: crate::schedule::ScheduleId,
    pub new_interpolation: crate::schedule::ScheduleInterpolation,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_daily_schedule_interpolation<Op: From<ChangeDailyScheduleInterpolation>>(id: crate::schedule::ScheduleId, new_interpolation: crate::schedule::ScheduleInterpolation) -> Op {
    ChangeDailyScheduleInterpolation { id, new_interpolation }.into()
}
//#endregion 🔖️Mutation
