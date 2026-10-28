//! 🗒️ Energy model mutation — `DeleteSpaceList`: Removes one space grouping. The spaces themselves are untouched — a list owns membership, not the members.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗒️ `delete-space-list` payload. Removes one space grouping. The spaces themselves are untouched — a list owns membership, not the members.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-space-list")]
pub struct DeleteSpaceList {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_space_list<Op: From<DeleteSpaceList>>(id: crate::model::EntityId) -> Op {
    DeleteSpaceList { id }.into()
}

//#endregion 🔖️Mutation
