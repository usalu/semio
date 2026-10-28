//! 🚿️ Energy model mutation — `ChangeMechanicalVentilationDesignFlow`: Sets design supply flow (m³/s) on one mechanical ventilation, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚿️ `change-mechanical-ventilation-design-flow` payload. Sets design supply flow (m³/s) on one mechanical ventilation, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-design-flow")]
pub struct ChangeMechanicalVentilationDesignFlow {
    pub id: crate::model::EntityId,
    pub new_design_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_mechanical_ventilation_design_flow<Op: From<ChangeMechanicalVentilationDesignFlow>>(id: crate::model::EntityId, new_design_flow_m3_s: f64) -> Op {
    ChangeMechanicalVentilationDesignFlow { id, new_design_flow_m3_s }.into()
}
//#endregion 🔖️Mutation
