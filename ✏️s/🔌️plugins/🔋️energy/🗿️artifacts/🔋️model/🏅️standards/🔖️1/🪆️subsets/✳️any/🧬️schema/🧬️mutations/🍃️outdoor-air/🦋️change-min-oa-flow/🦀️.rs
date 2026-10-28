//! 🦋️ Energy model mutation — `ChangeOutdoorAirSystemMinOaFlow`: Sets the outdoor air flow the system always draws, in m³/s.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🦋️ `change-outdoor-air-system-min-oa-flow` payload. Sets the outdoor air flow the system always draws, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-min-oa-flow")]
pub struct ChangeOutdoorAirSystemMinOaFlow {
    pub id: crate::model::EntityId,
    pub new_min_oa_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_outdoor_air_system_min_oa_flow<Op: From<ChangeOutdoorAirSystemMinOaFlow>>(id: crate::model::EntityId, new_min_oa_flow_m3_s: f64) -> Op {
    ChangeOutdoorAirSystemMinOaFlow { id, new_min_oa_flow_m3_s }.into()
}
//#endregion 🔖️Mutation
