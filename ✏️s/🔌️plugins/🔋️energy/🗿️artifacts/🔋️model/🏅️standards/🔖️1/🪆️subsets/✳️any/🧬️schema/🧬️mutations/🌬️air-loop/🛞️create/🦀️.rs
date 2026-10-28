//! 🛞️ Energy model mutation — `CreateAirLoop`: Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛞️ `create-air-loop` payload. Creates one air loop with its supply and return node ids, its design supply air flow and the zones its terminals serve. The terminal list is kept ascending and free of duplicates so a later membership removal's undo is exact.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-air-loop")]
pub struct CreateAirLoop {
    pub id: crate::model::EntityId,
    pub name: String,
    pub supply_node_id: u32,
    pub return_node_id: u32,
    pub design_supply_air_flow_m3_s: f64,
    pub terminal_zone_ids: Vec<crate::model::EntityId>,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_air_loop<Op: From<CreateAirLoop>>(id: crate::model::EntityId, name: String, supply_node_id: u32, return_node_id: u32, design_supply_air_flow_m3_s: f64, terminal_zone_ids: Vec<crate::model::EntityId>, index: Option<u32>) -> Op {
    CreateAirLoop { id, name, supply_node_id, return_node_id, design_supply_air_flow_m3_s, terminal_zone_ids, index }.into()
}
//#endregion 🔖️Mutation
