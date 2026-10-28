//! 🌪️ Energy model mutation — `CreateMechanicalVentilation`: Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌪️ `create-mechanical-ventilation` payload. Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-mechanical-ventilation")]
pub struct CreateMechanicalVentilation {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::schedule::ScheduleId,
    pub design_flow_m3_s: f64,
    pub fan_total_efficiency: f64,
    pub fan_delta_pressure_pa: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_mechanical_ventilation<Op: From<CreateMechanicalVentilation>>(
    index: u32,
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    schedule_id: crate::schedule::ScheduleId,
    design_flow_m3_s: f64,
    fan_total_efficiency: f64,
    fan_delta_pressure_pa: f64,
) -> Op {
    CreateMechanicalVentilation { index, id, zone_id, schedule_id, design_flow_m3_s, fan_total_efficiency, fan_delta_pressure_pa }.into()
}
//#endregion 🔖️Mutation
