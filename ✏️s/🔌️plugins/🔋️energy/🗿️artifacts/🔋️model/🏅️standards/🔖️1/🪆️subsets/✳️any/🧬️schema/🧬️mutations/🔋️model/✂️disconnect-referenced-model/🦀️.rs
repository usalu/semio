//! ✂️ Energy model mutation — `DisconnectReferencedModel`: Removes the relationship to the geometry model. Refused when no relationship exists.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ✂️ `disconnect-referenced-model` payload. Removes the relationship to the geometry model. Refused when no relationship exists.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-referenced-model")]
pub struct DisconnectReferencedModel {}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn disconnect_referenced_model<Op: From<DisconnectReferencedModel>>() -> Op {
    DisconnectReferencedModel {}.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
