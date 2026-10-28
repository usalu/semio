//! 🚤️ Energy model mutation — `ChangePlantLoopDesignFlow`: Sets the mass flow the loop is designed around, in kg/s.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚤️ `change-plant-loop-design-flow` payload. Sets the mass flow the loop is designed around, in kg/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-design-flow")]
pub struct ChangePlantLoopDesignFlow {
    pub id: crate::model::EntityId,
    pub new_design_flow_kg_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_plant_loop_design_flow<Op: From<ChangePlantLoopDesignFlow>>(id: crate::model::EntityId, new_design_flow_kg_s: f64) -> Op {
    ChangePlantLoopDesignFlow { id, new_design_flow_kg_s }.into()
}
//#endregion 🔖️Mutation
