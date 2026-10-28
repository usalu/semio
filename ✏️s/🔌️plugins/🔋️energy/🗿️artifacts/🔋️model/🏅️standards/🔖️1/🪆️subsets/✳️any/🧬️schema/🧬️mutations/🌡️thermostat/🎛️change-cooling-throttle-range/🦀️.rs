//! 🎛️ Energy model mutation — `ChangeThermostatCoolingThrottleRange`: Sets the proportional band the cooling setpoint is approached over, in kelvin.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎛️ `change-thermostat-cooling-throttle-range` payload. Sets the proportional band the cooling setpoint is approached over, in kelvin.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-cooling-throttle-range")]
pub struct ChangeThermostatCoolingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_cooling_throttle_range_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_thermostat_cooling_throttle_range<Op: From<ChangeThermostatCoolingThrottleRange>>(id: crate::model::EntityId, new_cooling_throttle_range_k: f64) -> Op {
    ChangeThermostatCoolingThrottleRange { id, new_cooling_throttle_range_k }.into()
}
//#endregion 🔖️Mutation
