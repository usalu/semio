//! 🕘️ Energy model mutation — `ReplaceTimeSeriesScheduleValues`: Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕘️ `replace-time-series-schedule-values` payload. Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-time-series-schedule-values")]
pub struct ReplaceTimeSeriesScheduleValues {
    pub id: crate::schedule::ScheduleId,
    pub new_values: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_time_series_schedule_values<Op: From<ReplaceTimeSeriesScheduleValues>>(id: crate::schedule::ScheduleId, new_values: Vec<f64>) -> Op {
    ReplaceTimeSeriesScheduleValues { id, new_values }.into()
}
//#endregion 🔖️Mutation
