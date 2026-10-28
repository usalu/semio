//! 🕞️ Energy model mutation — `CreateDailySchedule`: Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕞️ `create-daily-schedule` payload. Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-daily-schedule")]
pub struct CreateDailySchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub hourly_values: Vec<f64>,
    pub interpolation: crate::schedule::ScheduleInterpolation,
    pub limits_min: Option<f64>,
    pub limits_max: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_daily_schedule<Op: From<CreateDailySchedule>>(index: u32, id: crate::schedule::ScheduleId, hourly_values: Vec<f64>, interpolation: crate::schedule::ScheduleInterpolation, limits_min: Option<f64>, limits_max: Option<f64>) -> Op {
    CreateDailySchedule { index, id, hourly_values, interpolation, limits_min, limits_max }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
