//! 🕑️ Energy model mutation — `ChangeRefrigerationSystemDefrostSchedule`: Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕑️ `change-refrigeration-system-defrost-schedule` payload. Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-defrost-schedule")]
pub struct ChangeRefrigerationSystemDefrostSchedule {
    pub id: crate::model::EntityId,
    pub new_defrost_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_refrigeration_system_defrost_schedule<Op: From<ChangeRefrigerationSystemDefrostSchedule>>(id: crate::model::EntityId, new_defrost_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeRefrigerationSystemDefrostSchedule { id, new_defrost_schedule_id }.into()
}
//#endregion 🔖️Mutation
