//! 🍥️ Energy model mutation — `ChangeAirLoopDesignSupplyAirFlow`: Sets the volumetric air flow the loop is designed around, in m³/s.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍥️ `change-air-loop-design-supply-air-flow` payload. Sets the volumetric air flow the loop is designed around, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-air-loop-design-supply-air-flow")]
pub struct ChangeAirLoopDesignSupplyAirFlow {
    pub id: crate::model::EntityId,
    pub new_design_supply_air_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_air_loop_design_supply_air_flow<Op: From<ChangeAirLoopDesignSupplyAirFlow>>(id: crate::model::EntityId, new_design_supply_air_flow_m3_s: f64) -> Op {
    ChangeAirLoopDesignSupplyAirFlow { id, new_design_supply_air_flow_m3_s }.into()
}
//#endregion 🔖️Mutation
