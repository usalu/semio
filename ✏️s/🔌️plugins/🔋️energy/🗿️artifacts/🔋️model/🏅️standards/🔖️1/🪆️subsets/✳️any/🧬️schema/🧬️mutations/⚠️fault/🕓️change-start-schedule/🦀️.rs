//! 🕓️ Energy model mutation — `ChangeFaultStartSchedule`: Re-points one fault's schedule reference; a schedule the document does not define is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕓️ `change-fault-start-schedule` payload. Re-points one fault's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-start-schedule")]
pub struct ChangeFaultStartSchedule {
    pub id: crate::model::EntityId,
    pub new_start_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fault_start_schedule<Op: From<ChangeFaultStartSchedule>>(id: crate::model::EntityId, new_start_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeFaultStartSchedule { id, new_start_schedule_id }.into()
}
//#endregion 🔖️Mutation
