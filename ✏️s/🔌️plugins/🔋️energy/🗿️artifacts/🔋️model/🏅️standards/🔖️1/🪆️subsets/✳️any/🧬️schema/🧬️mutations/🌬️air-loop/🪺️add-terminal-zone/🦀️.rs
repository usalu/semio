//! 🪺️ Energy model mutation — `AddAirLoopTerminalZone`: Puts one more zone on the air loop's terminal list, at its ascending position.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪺️ `add-air-loop-terminal-zone` payload. Puts one more zone on the air loop's terminal list, at its ascending position.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-air-loop-terminal-zone")]
pub struct AddAirLoopTerminalZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_air_loop_terminal_zone<Op: From<AddAirLoopTerminalZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId, index: Option<u32>) -> Op {
    AddAirLoopTerminalZone { id, zone_id, index }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
