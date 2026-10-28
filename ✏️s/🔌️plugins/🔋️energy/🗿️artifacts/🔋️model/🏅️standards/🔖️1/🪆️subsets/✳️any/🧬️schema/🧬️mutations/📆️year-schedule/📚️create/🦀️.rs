//! 📚️ Energy model mutation — `CreateAnnualSchedule`: Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📚️ `create-annual-schedule` payload. Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-annual-schedule")]
pub struct CreateAnnualSchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub default_daily_schedule_id: crate::schedule::ScheduleId,
    pub holiday_daily_schedule_id: Option<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_annual_schedule<Op: From<CreateAnnualSchedule>>(index: u32, id: crate::schedule::ScheduleId, default_daily_schedule_id: crate::schedule::ScheduleId, holiday_daily_schedule_id: Option<crate::schedule::ScheduleId>) -> Op {
    CreateAnnualSchedule { index, id, default_daily_schedule_id, holiday_daily_schedule_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
