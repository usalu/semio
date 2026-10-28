//! 🚪️ Energy model mutation — `DeleteFenestration`: Removes one fenestration. Nothing in the document references a fenestration, so it neither cascades nor restricts.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚪️ `delete-fenestration` payload. Removes one fenestration. Nothing in the document references a fenestration, so it neither cascades nor restricts.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-fenestration")]
pub struct DeleteFenestration {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_fenestration<Op: From<DeleteFenestration>>(id: crate::model::EntityId) -> Op {
    DeleteFenestration { id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
