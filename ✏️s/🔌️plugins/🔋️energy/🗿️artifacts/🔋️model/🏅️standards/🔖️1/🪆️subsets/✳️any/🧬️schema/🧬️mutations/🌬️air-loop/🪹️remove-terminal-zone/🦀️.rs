//! 🪹️ Energy model mutation — `RemoveAirLoopTerminalZone`: Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪹️ `remove-air-loop-terminal-zone` payload. Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-air-loop-terminal-zone")]
pub struct RemoveAirLoopTerminalZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_air_loop_terminal_zone<Op: From<RemoveAirLoopTerminalZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> Op {
    RemoveAirLoopTerminalZone { id, zone_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
