//! 🎐️ Energy model mutation — `ChangeLightingGainReturnAirFraction`: Sets return-air fraction on one lighting gain, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎐️ `change-lighting-gain-return-air-fraction` payload. Sets return-air fraction on one lighting gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-lighting-gain-return-air-fraction")]
pub struct ChangeLightingGainReturnAirFraction {
    pub id: crate::model::EntityId,
    pub new_return_air_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_lighting_gain_return_air_fraction<Op: From<ChangeLightingGainReturnAirFraction>>(id: crate::model::EntityId, new_return_air_fraction: f64) -> Op {
    ChangeLightingGainReturnAirFraction { id, new_return_air_fraction }.into()
}
//#endregion 🔖️Mutation
