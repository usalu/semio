//! 🕜️ Energy model mutation — `CreateConstantSchedule`: Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕜️ `create-constant-schedule` payload. Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-constant-schedule")]
pub struct CreateConstantSchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub value: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_constant_schedule<Op: From<CreateConstantSchedule>>(index: u32, id: crate::schedule::ScheduleId, value: f64) -> Op {
    CreateConstantSchedule { index, id, value }.into()
}
//#endregion 🔖️Mutation

#[cfg(test)]
#[path="🧪️native/🦀️.rs"]mod native_tests;
