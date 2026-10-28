//! 🧻️ Energy model mutation — `ChangeHumidistatDehumidifyingThrottleRange`: Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧻️ `change-humidistat-dehumidifying-throttle-range` payload. Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-dehumidifying-throttle-range")]
pub struct ChangeHumidistatDehumidifyingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_dehumidifying_throttle_range: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_humidistat_dehumidifying_throttle_range<Op: From<ChangeHumidistatDehumidifyingThrottleRange>>(id: crate::model::EntityId, new_dehumidifying_throttle_range: f64) -> Op {
    ChangeHumidistatDehumidifyingThrottleRange { id, new_dehumidifying_throttle_range }.into()
}
//#endregion 🔖️Mutation
