//! ↗️ Energy model mutation — `ChangeAirLoopSupplyNode`: Sets the node the air loop supplies from. Node zero is the unset reading, so it is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ↗️ `change-air-loop-supply-node` payload. Sets the node the air loop supplies from. Node zero is the unset reading, so it is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-air-loop-supply-node")]
pub struct ChangeAirLoopSupplyNode {
    pub id: crate::model::EntityId,
    pub new_supply_node_id: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_air_loop_supply_node<Op: From<ChangeAirLoopSupplyNode>>(id: crate::model::EntityId, new_supply_node_id: u32) -> Op {
    ChangeAirLoopSupplyNode { id, new_supply_node_id }.into()
}
//#endregion 🔖️Mutation
