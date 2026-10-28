//! 🕔️ Energy model mutation — `ReplaceDailyScheduleHourlyValues`: Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕔️ `replace-daily-schedule-hourly-values` payload. Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-daily-schedule-hourly-values")]
pub struct ReplaceDailyScheduleHourlyValues {
    pub id: crate::schedule::ScheduleId,
    pub new_hourly_values: [f64;24],
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_daily_schedule_hourly_values<Op: From<ReplaceDailyScheduleHourlyValues>>(id: crate::schedule::ScheduleId, new_hourly_values: [f64;24]) -> Op {
    ReplaceDailyScheduleHourlyValues { id, new_hourly_values }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
