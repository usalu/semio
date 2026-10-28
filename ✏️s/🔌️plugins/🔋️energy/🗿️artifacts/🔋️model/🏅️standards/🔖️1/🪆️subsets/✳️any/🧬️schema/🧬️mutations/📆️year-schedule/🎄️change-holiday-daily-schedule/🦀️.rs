//! 🎄️ Energy model mutation — `ChangeAnnualScheduleHolidayDailySchedule`: Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎄️ `change-annual-schedule-holiday-daily-schedule` payload. Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-annual-schedule-holiday-daily-schedule")]
pub struct ChangeAnnualScheduleHolidayDailySchedule {
    pub id: crate::schedule::ScheduleId,
    pub new_holiday_daily_schedule_id: Option<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_annual_schedule_holiday_daily_schedule<Op: From<ChangeAnnualScheduleHolidayDailySchedule>>(id: crate::schedule::ScheduleId, new_holiday_daily_schedule_id: Option<crate::schedule::ScheduleId>) -> Op {
    ChangeAnnualScheduleHolidayDailySchedule { id, new_holiday_daily_schedule_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
