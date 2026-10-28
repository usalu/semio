//! 🕯️ Energy model mutation — `DeleteLightingGain`: Removes one lighting gain.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕯️ `delete-lighting-gain` payload. Removes one lighting gain.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-lighting-gain")]
pub struct DeleteLightingGain {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_lighting_gain<Op: From<DeleteLightingGain>>(id: crate::model::EntityId) -> Op {
    DeleteLightingGain { id }.into()
}

//#endregion 🔖️Mutation
