//! 🎊️ Energy model mutation — `RemoveAnnualScheduleHoliday`: Takes one calendar date back out of a year's holiday set, so lookups on it fall back to the matching rule again.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎊️ `remove-annual-schedule-holiday` payload. Takes one calendar date back out of a year's holiday set, so lookups on it fall back to the matching rule again.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-annual-schedule-holiday")]
pub struct RemoveAnnualScheduleHoliday {
    pub id: crate::schedule::ScheduleId,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_annual_schedule_holiday<Op: From<RemoveAnnualScheduleHoliday>>(id: crate::schedule::ScheduleId, year: u16, month: u8, day: u8) -> Op {
    RemoveAnnualScheduleHoliday { id, year, month, day }.into()
}

//#endregion 🔖️Mutation
