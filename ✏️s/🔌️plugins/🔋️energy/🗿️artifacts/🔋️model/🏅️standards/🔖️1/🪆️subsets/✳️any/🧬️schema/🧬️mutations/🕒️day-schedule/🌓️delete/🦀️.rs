//! 🌓️ Energy model mutation — `DeleteDailySchedule`: Removes one daily profile. Refused while a weekly or annual schedule — or any consumer — still names it.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌓️ `delete-daily-schedule` payload. Removes one daily profile. Refused while a weekly or annual schedule — or any consumer — still names it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-daily-schedule")]
pub struct DeleteDailySchedule {
    pub id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_daily_schedule<Op: From<DeleteDailySchedule>>(id: crate::schedule::ScheduleId) -> Op {
    DeleteDailySchedule { id }.into()
}

//#endregion 🔖️Mutation
