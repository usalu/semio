//! 🧢️ Energy model mutation — `ChangeFenestrationOverhangDepth`: Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧢️ `change-fenestration-overhang-depth` payload. Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-overhang-depth")]
pub struct ChangeFenestrationOverhangDepth {
    pub id: crate::model::EntityId,
    pub new_overhang_depth_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_overhang_depth<Op: From<ChangeFenestrationOverhangDepth>>(id: crate::model::EntityId, new_overhang_depth_m: f64) -> Op {
    ChangeFenestrationOverhangDepth { id, new_overhang_depth_m }.into()
}
//#endregion 🔖️Mutation
