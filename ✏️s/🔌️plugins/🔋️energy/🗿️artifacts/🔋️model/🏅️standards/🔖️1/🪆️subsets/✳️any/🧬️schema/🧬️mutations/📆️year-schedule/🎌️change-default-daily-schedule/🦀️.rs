//! 🎌️ Energy model mutation — `ChangeAnnualScheduleDefaultDailySchedule`: Re-points the profile a year falls back to on every day no rule matches.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎌️ `change-annual-schedule-default-daily-schedule` payload. Re-points the profile a year falls back to on every day no rule matches.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-annual-schedule-default-daily-schedule")]
pub struct ChangeAnnualScheduleDefaultDailySchedule {
    pub id: crate::schedule::ScheduleId,
    pub new_default_daily_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_annual_schedule_default_daily_schedule<Op: From<ChangeAnnualScheduleDefaultDailySchedule>>(id: crate::schedule::ScheduleId, new_default_daily_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeAnnualScheduleDefaultDailySchedule { id, new_default_daily_schedule_id }.into()
}
//#endregion 🔖️Mutation
