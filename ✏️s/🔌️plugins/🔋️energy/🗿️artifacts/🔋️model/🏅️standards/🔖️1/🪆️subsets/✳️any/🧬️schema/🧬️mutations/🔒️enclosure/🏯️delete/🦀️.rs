//! 🏯️ Energy model mutation — `DeleteThermalEnclosure`: Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏯️ `delete-thermal-enclosure` payload. Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-thermal-enclosure")]
pub struct DeleteThermalEnclosure {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_thermal_enclosure<Op: From<DeleteThermalEnclosure>>(id: crate::model::EntityId) -> Op {
    DeleteThermalEnclosure { id }.into()
}

//#endregion 🔖️Mutation
