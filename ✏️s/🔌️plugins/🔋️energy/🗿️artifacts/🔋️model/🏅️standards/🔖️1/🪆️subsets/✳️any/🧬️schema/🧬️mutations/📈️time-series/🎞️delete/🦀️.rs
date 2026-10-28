//! 🎞️ Energy model mutation — `DeleteTimeSeriesSchedule`: Removes one measured series. Refused while any consumer still resolves its id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎞️ `delete-time-series-schedule` payload. Removes one measured series. Refused while any consumer still resolves its id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-time-series-schedule")]
pub struct DeleteTimeSeriesSchedule {
    pub id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_time_series_schedule<Op: From<DeleteTimeSeriesSchedule>>(id: crate::schedule::ScheduleId) -> Op {
    DeleteTimeSeriesSchedule { id }.into()
}

//#endregion 🔖️Mutation
