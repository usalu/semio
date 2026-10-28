//! 🔀️ Energy model mutation — `ReorderConstructionLayers`: Restates a construction's whole layer sequence. The new sequence must be a permutation of the one the construction already holds — adding or dropping a layer is `add-construction-layer`'s and `remove-construction-layer`'s job, so a reorder can never change which materials a wall is made of.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔀️ `reorder-construction-layers` payload. Restates a construction's whole layer sequence. The new sequence must be a permutation of the one the construction already holds — adding or dropping a layer is `add-construction-layer`'s and `remove-construction-layer`'s job, so a reorder can never change which materials a wall is made of.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-construction-layers")]
pub struct ReorderConstructionLayers {
    pub id: crate::model::EntityId,
    pub new_layer_material_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn reorder_construction_layers<Op: From<ReorderConstructionLayers>>(id: crate::model::EntityId, new_layer_material_ids: Vec<crate::model::EntityId>) -> Op {
    ReorderConstructionLayers { id, new_layer_material_ids }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
