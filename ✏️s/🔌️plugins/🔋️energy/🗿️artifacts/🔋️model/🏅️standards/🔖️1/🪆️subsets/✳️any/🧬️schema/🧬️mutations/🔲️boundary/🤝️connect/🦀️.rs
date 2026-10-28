//! 🤝️ Energy model mutation — `ConnectSurfaces`: Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🤝️ `connect-surfaces` payload. Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-surfaces")]
pub struct ConnectSurfaces {
    pub surface_a_id: crate::model::EntityId,
    pub surface_b_id: crate::model::EntityId,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn connect_surfaces<Op: From<ConnectSurfaces>>(surface_a_id: crate::model::EntityId, surface_b_id: crate::model::EntityId, index: Option<u32>) -> Op {
    ConnectSurfaces { surface_a_id, surface_b_id, index }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
