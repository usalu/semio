//! 🫥️ Energy model mutation — `DeleteIdealLoadsSystem`: Drops the ideal-loads air system off its zone. Refused when no system carries the id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫥️ `delete-ideal-loads-system` payload. Drops the ideal-loads air system off its zone. Refused when no system carries the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-ideal-loads-system")]
pub struct DeleteIdealLoadsSystem {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_ideal_loads_system<Op: From<DeleteIdealLoadsSystem>>(id: crate::model::EntityId) -> Op {
    DeleteIdealLoadsSystem { id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
