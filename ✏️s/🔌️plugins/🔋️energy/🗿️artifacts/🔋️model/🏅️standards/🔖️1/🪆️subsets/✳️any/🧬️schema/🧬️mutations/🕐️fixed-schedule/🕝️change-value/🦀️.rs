//! 🕝️ Energy model mutation — `ChangeConstantScheduleValue`: Sets the one value a constant schedule returns at every timestep.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕝️ `change-constant-schedule-value` payload. Sets the one value a constant schedule returns at every timestep.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-constant-schedule-value")]
pub struct ChangeConstantScheduleValue {
    pub id: crate::schedule::ScheduleId,
    pub new_value: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_constant_schedule_value<Op: From<ChangeConstantScheduleValue>>(id: crate::schedule::ScheduleId, new_value: f64) -> Op {
    ChangeConstantScheduleValue { id, new_value }.into()
}
//#endregion 🔖️Mutation
