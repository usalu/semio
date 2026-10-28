//! 🕗️ Energy model mutation — `ChangeWeeklyScheduleDay`: Points one day of a week at a different daily profile. The address is the pair (schedule id, day index 0 Sunday … 6 Saturday); a day outside the week and a profile the document does not define are both refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕗️ `change-weekly-schedule-day` payload. Points one day of a week at a different daily profile. The address is the pair (schedule id, day index 0 Sunday … 6 Saturday); a day outside the week and a profile the document does not define are both refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-weekly-schedule-day")]
pub struct ChangeWeeklyScheduleDay {
    pub id: crate::schedule::ScheduleId,
    pub day_index: u8,
    pub new_daily_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_weekly_schedule_day<Op: From<ChangeWeeklyScheduleDay>>(id: crate::schedule::ScheduleId, day_index: u8, new_daily_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeWeeklyScheduleDay { id, day_index, new_daily_schedule_id }.into()
}

//#endregion 🔖️Mutation
