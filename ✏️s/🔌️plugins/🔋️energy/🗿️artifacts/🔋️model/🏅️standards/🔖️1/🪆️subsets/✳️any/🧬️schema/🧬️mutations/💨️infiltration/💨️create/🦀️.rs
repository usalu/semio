//! 💨️ Energy model mutation — `CreateInfiltration`: Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💨️ `create-infiltration` payload. Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-infiltration")]
pub struct CreateInfiltration {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::schedule::ScheduleId,
    pub method: crate::air_exchange::InfiltrationMethod,
    pub design_flow_ach: f64,
    pub flow_per_exterior_area_m3_s_m2: f64,
    pub effective_leakage_area_m2: f64,
    pub discharge_coefficient: f64,
    pub stack_height_m: f64,
    pub constant_term_coefficient: f64,
    pub temperature_term_coefficient: f64,
    pub velocity_term_coefficient: f64,
    pub velocity_squared_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_infiltration<Op: From<CreateInfiltration>>(
    index: u32,
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    schedule_id: crate::schedule::ScheduleId,
    method: crate::air_exchange::InfiltrationMethod,
    design_flow_ach: f64,
    flow_per_exterior_area_m3_s_m2: f64,
    effective_leakage_area_m2: f64,
    discharge_coefficient: f64,
    stack_height_m: f64,
    constant_term_coefficient: f64,
    temperature_term_coefficient: f64,
    velocity_term_coefficient: f64,
    velocity_squared_term_coefficient: f64,
) -> Op {
    CreateInfiltration {
        index,
        id,
        zone_id,
        schedule_id,
        method,
        design_flow_ach,
        flow_per_exterior_area_m3_s_m2,
        effective_leakage_area_m2,
        discharge_coefficient,
        stack_height_m,
        constant_term_coefficient,
        temperature_term_coefficient,
        velocity_term_coefficient,
        velocity_squared_term_coefficient,
    }.into()
}
//#endregion 🔖️Mutation
