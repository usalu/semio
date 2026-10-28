//! 🗓️ Energy model mutation — `CreateWeeklySchedule`: Defines one week as seven daily profiles, Sunday first (slot 0) through Saturday (slot 6) — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗓️ `create-weekly-schedule` payload. Defines one week as seven daily profiles, Sunday first (slot 0) through Saturday (slot 6) — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-weekly-schedule")]
pub struct CreateWeeklySchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub daily_schedule_ids: Vec<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_weekly_schedule<Op: From<CreateWeeklySchedule>>(index: u32, id: crate::schedule::ScheduleId, daily_schedule_ids: Vec<crate::schedule::ScheduleId>) -> Op {
    CreateWeeklySchedule { index, id, daily_schedule_ids }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
