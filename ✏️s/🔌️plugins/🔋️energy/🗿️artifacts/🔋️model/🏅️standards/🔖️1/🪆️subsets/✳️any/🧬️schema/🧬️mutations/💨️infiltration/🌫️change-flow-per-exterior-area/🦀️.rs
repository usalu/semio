//! 🌫️ Energy model mutation — `ChangeInfiltrationFlowPerExteriorArea`: Sets flow per exterior area (m³/s·m²) on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌫️ `change-infiltration-flow-per-exterior-area` payload. Sets flow per exterior area (m³/s·m²) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-flow-per-exterior-area")]
pub struct ChangeInfiltrationFlowPerExteriorArea {
    pub id: crate::model::EntityId,
    pub new_flow_per_exterior_area_m3_s_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_flow_per_exterior_area<Op: From<ChangeInfiltrationFlowPerExteriorArea>>(id: crate::model::EntityId, new_flow_per_exterior_area_m3_s_m2: f64) -> Op {
    ChangeInfiltrationFlowPerExteriorArea { id, new_flow_per_exterior_area_m3_s_m2 }.into()
}
//#endregion 🔖️Mutation
