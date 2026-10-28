//! 🧹️ Energy model mutation — `DeleteSpace`: Removes one space. Refused while a space list still names it, so no grouping is left pointing at nothing.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧹️ `delete-space` payload. Removes one space. Refused while a space list still names it, so no grouping is left pointing at nothing.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-space")]
pub struct DeleteSpace {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_space<Op: From<DeleteSpace>>(id: crate::model::EntityId) -> Op {
    DeleteSpace { id }.into()
}

//#endregion 🔖️Mutation
