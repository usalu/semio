//! 🪤️ Energy model mutation — `RemoveSpaceListMember`: Takes one space back out of a grouping; the space itself survives.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪤️ `remove-space-list-member` payload. Takes one space back out of a grouping; the space itself survives.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-space-list-member")]
pub struct RemoveSpaceListMember {
    pub id: crate::model::EntityId,
    pub space_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_space_list_member<Op: From<RemoveSpaceListMember>>(id: crate::model::EntityId, space_id: crate::model::EntityId) -> Op {
    RemoveSpaceListMember { id, space_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
