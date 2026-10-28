//! 🕖️ Energy model mutation — `DeleteWeeklySchedule`: Removes one weekly schedule. Refused while any consumer still resolves its id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕖️ `delete-weekly-schedule` payload. Removes one weekly schedule. Refused while any consumer still resolves its id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-weekly-schedule")]
pub struct DeleteWeeklySchedule {
    pub id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_weekly_schedule<Op: From<DeleteWeeklySchedule>>(id: crate::schedule::ScheduleId) -> Op {
    DeleteWeeklySchedule { id }.into()
}

//#endregion 🔖️Mutation
