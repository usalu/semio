//! 🔴️ Energy model mutation — `ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp`: Sets the warmest air the ideal-loads system may deliver, in °C.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` payload. Sets the warmest air the ideal-loads system may deliver, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-heating-supply-air-temp")]
pub struct ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp {
    pub id: crate::model::EntityId,
    pub new_max_heating_supply_air_temp_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_max_heating_supply_air_temp<Op: From<ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp>>(id: crate::model::EntityId, new_max_heating_supply_air_temp_c: f64) -> Op {
    ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp { id, new_max_heating_supply_air_temp_c }.into()
}
//#endregion 🔖️Mutation
