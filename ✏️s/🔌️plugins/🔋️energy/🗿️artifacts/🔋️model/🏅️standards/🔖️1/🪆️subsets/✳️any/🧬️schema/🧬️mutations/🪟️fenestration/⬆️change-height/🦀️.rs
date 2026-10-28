//! ⬆️ Energy model mutation — `ChangeFenestrationHeight`: Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⬆️ `change-fenestration-height` payload. Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-height")]
pub struct ChangeFenestrationHeight {
    pub id: crate::model::EntityId,
    pub new_height_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_height<Op: From<ChangeFenestrationHeight>>(id: crate::model::EntityId, new_height_m: f64) -> Op {
    ChangeFenestrationHeight { id, new_height_m }.into()
}
//#endregion 🔖️Mutation
