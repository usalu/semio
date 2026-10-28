//! 🪢️ Energy model mutation — `ConnectReferencedModel`: Creates the relationship between this energy model and the geometry model it was derived from, with its complete target, history pin and relationship role.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪢️ `connect-referenced-model` payload. Creates the relationship between this energy model and the geometry model it was derived from, with its complete target, history pin and relationship role.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-referenced-model")]
pub struct ConnectReferencedModel {
    pub link: protocol::ArtifactLink,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn connect_referenced_model<Op: From<ConnectReferencedModel>>(link: protocol::ArtifactLink) -> Op {
    ConnectReferencedModel { link }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
