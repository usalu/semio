//! 🔄️ Energy model mutation — `ChangeInfiltrationDesignFlowAch`: Sets design flow (air changes per hour) on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔄️ `change-infiltration-design-flow-ach` payload. Sets design flow (air changes per hour) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-design-flow-ach")]
pub struct ChangeInfiltrationDesignFlowAch {
    pub id: crate::model::EntityId,
    pub new_design_flow_ach: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_design_flow_ach<Op: From<ChangeInfiltrationDesignFlowAch>>(id: crate::model::EntityId, new_design_flow_ach: f64) -> Op {
    ChangeInfiltrationDesignFlowAch { id, new_design_flow_ach }.into()
}
//#endregion 🔖️Mutation
