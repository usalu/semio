//! ➖️ Energy model mutation — `RemoveConstructionLayer`: Removes the material layer a construction holds at a stated position. Addressed by position rather than by material id because the same material may legitimately appear in a construction more than once.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ➖️ `remove-construction-layer` payload. Removes the material layer a construction holds at a stated position. Addressed by position rather than by material id because the same material may legitimately appear in a construction more than once.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-construction-layer")]
pub struct RemoveConstructionLayer {
    pub id: crate::model::EntityId,
    pub index: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_construction_layer<Op: From<RemoveConstructionLayer>>(id: crate::model::EntityId, index: u32) -> Op {
    RemoveConstructionLayer { id, index }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
