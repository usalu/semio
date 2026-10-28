//! 🧫️ Energy model mutation — `ChangePlantLoopReturnTemperature`: Sets the temperature the loop returns at, in °C.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧫️ `change-plant-loop-return-temperature` payload. Sets the temperature the loop returns at, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-return-temperature")]
pub struct ChangePlantLoopReturnTemperature {
    pub id: crate::model::EntityId,
    pub new_return_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_plant_loop_return_temperature<Op: From<ChangePlantLoopReturnTemperature>>(id: crate::model::EntityId, new_return_temperature_c: f64) -> Op {
    ChangePlantLoopReturnTemperature { id, new_return_temperature_c }.into()
}
//#endregion 🔖️Mutation
