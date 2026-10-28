//! 🎈️ Energy model mutation — `ChangeMechanicalVentilationFanDeltaPressure`: Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎈️ `change-mechanical-ventilation-fan-delta-pressure` payload. Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-fan-delta-pressure")]
pub struct ChangeMechanicalVentilationFanDeltaPressure {
    pub id: crate::model::EntityId,
    pub new_fan_delta_pressure_pa: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_mechanical_ventilation_fan_delta_pressure<Op: From<ChangeMechanicalVentilationFanDeltaPressure>>(id: crate::model::EntityId, new_fan_delta_pressure_pa: f64) -> Op {
    ChangeMechanicalVentilationFanDeltaPressure { id, new_fan_delta_pressure_pa }.into()
}
//#endregion 🔖️Mutation
