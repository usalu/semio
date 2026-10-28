//! ☕️ Energy model mutation — `ChangePlantLoopSupplyTemperature`: Sets the temperature the loop supplies at, in °C.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☕️ `change-plant-loop-supply-temperature` payload. Sets the temperature the loop supplies at, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-supply-temperature")]
pub struct ChangePlantLoopSupplyTemperature {
    pub id: crate::model::EntityId,
    pub new_supply_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_plant_loop_supply_temperature<Op: From<ChangePlantLoopSupplyTemperature>>(id: crate::model::EntityId, new_supply_temperature_c: f64) -> Op {
    ChangePlantLoopSupplyTemperature { id, new_supply_temperature_c }.into()
}
//#endregion 🔖️Mutation
