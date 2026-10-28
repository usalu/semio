//! 🌦️ Energy model mutation — `BindWeatherFile`: Attaches the `weather` link slot to an `🌦️epw` stdio artifact with its complete target, history pin and role; the model never embeds weather bytes.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌦️ `bind-weather-file` payload. Attaches the `weather` link slot to an `🌦️epw` stdio artifact with its complete target, history pin and role; the model never embeds weather bytes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "bind-weather-file")]
pub struct BindWeatherFile {
    pub link: protocol::ArtifactLink,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn bind_weather_file<Op: From<BindWeatherFile>>(link: protocol::ArtifactLink) -> Op {
    BindWeatherFile { link }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
