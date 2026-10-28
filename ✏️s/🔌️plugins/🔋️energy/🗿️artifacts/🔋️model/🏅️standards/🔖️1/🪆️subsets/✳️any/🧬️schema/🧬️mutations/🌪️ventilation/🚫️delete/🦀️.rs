//! 🚫️ Energy model mutation — `DeleteMechanicalVentilation`: Removes one mechanical ventilation specification.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚫️ `delete-mechanical-ventilation` payload. Removes one mechanical ventilation specification.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-mechanical-ventilation")]
pub struct DeleteMechanicalVentilation {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_mechanical_ventilation<Op: From<DeleteMechanicalVentilation>>(id: crate::model::EntityId) -> Op {
    DeleteMechanicalVentilation { id }.into()
}

//#endregion 🔖️Mutation
