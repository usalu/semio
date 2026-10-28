//! 🎚️ Energy model mutation — `ChangeThermostatHeatingThrottleRange`: Sets the proportional band the heating setpoint is approached over, in kelvin.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎚️ `change-thermostat-heating-throttle-range` payload. Sets the proportional band the heating setpoint is approached over, in kelvin.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-heating-throttle-range")]
pub struct ChangeThermostatHeatingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_heating_throttle_range_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_thermostat_heating_throttle_range<Op: From<ChangeThermostatHeatingThrottleRange>>(id: crate::model::EntityId, new_heating_throttle_range_k: f64) -> Op {
    ChangeThermostatHeatingThrottleRange { id, new_heating_throttle_range_k }.into()
}
//#endregion 🔖️Mutation
