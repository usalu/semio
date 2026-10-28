//! ➕️ Energy model mutation — `AddConstructionLayer`: Inserts one material layer into a construction at a stated position, outside-to-inside. Layer order is physically load-bearing — the same layers in a different order are a different wall — so the position is payload data, not an append.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ➕️ `add-construction-layer` payload. Inserts one material layer into a construction at a stated position, outside-to-inside. Layer order is physically load-bearing — the same layers in a different order are a different wall — so the position is payload data, not an append.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-construction-layer")]
pub struct AddConstructionLayer {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub material_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_construction_layer<Op: From<AddConstructionLayer>>(id: crate::model::EntityId, index: u32, material_id: crate::model::EntityId) -> Op {
    AddConstructionLayer { id, index, material_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
