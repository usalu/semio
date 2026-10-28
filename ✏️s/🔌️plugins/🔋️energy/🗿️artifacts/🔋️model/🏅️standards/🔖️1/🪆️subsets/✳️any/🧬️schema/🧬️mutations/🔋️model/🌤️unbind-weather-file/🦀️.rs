//! 🌤️ Energy model mutation — `UnbindWeatherFile`: Detaches the `weather` link slot. Refused when nothing is bound, so an undo chain can never invent an unbind that had no partner.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌤️ `unbind-weather-file` payload. Detaches the `weather` link slot. Refused when nothing is bound, so an undo chain can never invent an unbind that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "unbind-weather-file")]
pub struct UnbindWeatherFile {}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn unbind_weather_file<Op: From<UnbindWeatherFile>>() -> Op {
    UnbindWeatherFile {}.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
