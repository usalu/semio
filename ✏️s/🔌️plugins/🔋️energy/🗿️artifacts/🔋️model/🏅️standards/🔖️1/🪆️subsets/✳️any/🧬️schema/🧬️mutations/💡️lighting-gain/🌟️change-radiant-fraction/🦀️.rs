//! 🌟️ Energy model mutation — `ChangeLightingGainRadiantFraction`: Sets radiant fraction on one lighting gain, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌟️ `change-lighting-gain-radiant-fraction` payload. Sets radiant fraction on one lighting gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-lighting-gain-radiant-fraction")]
pub struct ChangeLightingGainRadiantFraction {
    pub id: crate::model::EntityId,
    pub new_radiant_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_lighting_gain_radiant_fraction<Op: From<ChangeLightingGainRadiantFraction>>(id: crate::model::EntityId, new_radiant_fraction: f64) -> Op {
    ChangeLightingGainRadiantFraction { id, new_radiant_fraction }.into()
}
//#endregion 🔖️Mutation
