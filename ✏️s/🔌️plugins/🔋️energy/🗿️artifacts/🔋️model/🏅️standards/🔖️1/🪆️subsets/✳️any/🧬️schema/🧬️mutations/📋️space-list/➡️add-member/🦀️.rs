//! ➡️ Energy model mutation — `AddSpaceListMember`: Puts one space into a grouping.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ➡️ `add-space-list-member` payload. Puts one space into a grouping.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-space-list-member")]
pub struct AddSpaceListMember {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub space_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_space_list_member<Op: From<AddSpaceListMember>>(id: crate::model::EntityId, index: u32, space_id: crate::model::EntityId) -> Op {
    AddSpaceListMember { id, index, space_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
