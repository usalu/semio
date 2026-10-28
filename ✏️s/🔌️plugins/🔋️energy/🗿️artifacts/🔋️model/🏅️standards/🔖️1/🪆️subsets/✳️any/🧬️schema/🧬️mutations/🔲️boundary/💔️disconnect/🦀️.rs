//! 💔️ Energy model mutation — `DisconnectSurfaces`: Removes the adjacency relationship between two surfaces, addressed by the unordered pair. Refused when the two are not adjacent, so an undo chain can never invent a disconnect that had no partner.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💔️ `disconnect-surfaces` payload. Removes the adjacency relationship between two surfaces, addressed by the unordered pair. Refused when the two are not adjacent, so an undo chain can never invent a disconnect that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-surfaces")]
pub struct DisconnectSurfaces {
    pub surface_a_id: crate::model::EntityId,
    pub surface_b_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn disconnect_surfaces<Op: From<DisconnectSurfaces>>(surface_a_id: crate::model::EntityId, surface_b_id: crate::model::EntityId) -> Op {
    DisconnectSurfaces { surface_a_id, surface_b_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
