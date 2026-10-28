//! 🚾️ Energy model mutation — `ChangeWaterSystemPeakFlow`: Sets peak flow (L/s) on one water system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚾️ `change-water-system-peak-flow` payload. Sets peak flow (L/s) on one water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-peak-flow")]
pub struct ChangeWaterSystemPeakFlow {
    pub id: crate::model::EntityId,
    pub new_peak_flow_l_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_water_system_peak_flow<Op: From<ChangeWaterSystemPeakFlow>>(id: crate::model::EntityId, new_peak_flow_l_s: f64) -> Op {
    ChangeWaterSystemPeakFlow { id, new_peak_flow_l_s }.into()
}
//#endregion 🔖️Mutation
