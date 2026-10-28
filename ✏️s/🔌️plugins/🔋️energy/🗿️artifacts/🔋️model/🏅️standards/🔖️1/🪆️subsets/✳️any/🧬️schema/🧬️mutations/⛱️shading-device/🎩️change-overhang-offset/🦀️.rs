//! 🎩️ Energy model mutation — `ChangeFenestrationOverhangOffset`: Sets how far above the window head the horizontal projection sits, in metres.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎩️ `change-fenestration-overhang-offset` payload. Sets how far above the window head the horizontal projection sits, in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-overhang-offset")]
pub struct ChangeFenestrationOverhangOffset {
    pub id: crate::model::EntityId,
    pub new_overhang_offset_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_overhang_offset<Op: From<ChangeFenestrationOverhangOffset>>(id: crate::model::EntityId, new_overhang_offset_m: f64) -> Op {
    ChangeFenestrationOverhangOffset { id, new_overhang_offset_m }.into()
}
//#endregion 🔖️Mutation
