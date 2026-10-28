//! 🪗️ Energy model mutation — `CreateTimeSeriesSchedule`: Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪗️ `create-time-series-schedule` payload. Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-time-series-schedule")]
pub struct CreateTimeSeriesSchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub values: Vec<f64>,
    pub timestep_seconds: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_time_series_schedule<Op: From<CreateTimeSeriesSchedule>>(index: u32, id: crate::schedule::ScheduleId, values: Vec<f64>, timestep_seconds: u32) -> Op {
    CreateTimeSeriesSchedule { index, id, values, timestep_seconds }.into()
}
//#endregion 🔖️Mutation

#[cfg(test)]
#[path="🧪️native/🦀️.rs"]mod native_tests;
