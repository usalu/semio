//! 🔃️ Energy model mutation — `ReplaceSetpointManagerKind`: Swaps the whole tagged control law of one setpoint manager. `replace`, not `change`, because the payload shape genuinely differs per variant (taxonomy rule for tagged unions); the four outdoor-air-reset limits are carried flattened beside the variant's wire name and must be zero for every other variant.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔃️ `replace-setpoint-manager-kind` payload. Swaps the whole tagged control law of one setpoint manager. `replace`, not `change`, because the payload shape genuinely differs per variant (taxonomy rule for tagged unions); the four outdoor-air-reset limits are carried flattened beside the variant's wire name and must be zero for every other variant.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-setpoint-manager-kind")]
pub struct ReplaceSetpointManagerKind {
    pub id: crate::model::EntityId,
    pub new_kind: crate::model::SetpointManagerKindTag,
    pub new_low_outdoor_c: f64,
    pub new_high_outdoor_c: f64,
    pub new_low_setpoint_c: f64,
    pub new_high_setpoint_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_setpoint_manager_kind<Op: From<ReplaceSetpointManagerKind>>(id: crate::model::EntityId, new_kind: crate::model::SetpointManagerKindTag, new_low_outdoor_c: f64, new_high_outdoor_c: f64, new_low_setpoint_c: f64, new_high_setpoint_c: f64) -> Op {
    ReplaceSetpointManagerKind { id, new_kind, new_low_outdoor_c, new_high_outdoor_c, new_low_setpoint_c, new_high_setpoint_c }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
