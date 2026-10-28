//! 🎉️ Energy model mutation — `AddAnnualScheduleHoliday`: Marks one calendar date as a holiday of a year, so lookups on it take the holiday profile instead of the matching rule. The dates are a set, but a JSON array positionally, so the payload carries the position the date takes.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎉️ `add-annual-schedule-holiday` payload. Marks one calendar date as a holiday of a year, so lookups on it take the holiday profile instead of the matching rule. The dates are a set, but a JSON array positionally, so the payload carries the position the date takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-annual-schedule-holiday")]
pub struct AddAnnualScheduleHoliday {
    pub id: crate::schedule::ScheduleId,
    pub index: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_annual_schedule_holiday<Op: From<AddAnnualScheduleHoliday>>(id: crate::schedule::ScheduleId, index: u32, year: u16, month: u8, day: u8) -> Op {
    AddAnnualScheduleHoliday { id, index, year, month, day }.into()
}

//#endregion 🔖️Mutation
