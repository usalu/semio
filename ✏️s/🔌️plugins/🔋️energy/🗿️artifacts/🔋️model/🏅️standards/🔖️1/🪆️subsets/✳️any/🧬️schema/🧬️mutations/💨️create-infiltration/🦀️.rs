//! 💨️ Energy model mutation — `CreateInfiltration`: Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💨️ `create-infiltration` payload. Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-infiltration")]
pub struct CreateInfiltration {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::model::ScheduleId,
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_infiltration(index: u32, id: crate::model::EntityId, zone_id: crate::model::EntityId, schedule_id: crate::model::ScheduleId, method: crate::air_exchange::InfiltrationMethod, design_flow_ach: f64, flow_per_exterior_area_m3_s_m2: f64, effective_leakage_area_m2: f64, discharge_coefficient: f64, stack_height_m: f64, constant_term_coefficient: f64, temperature_term_coefficient: f64, velocity_term_coefficient: f64, velocity_squared_term_coefficient: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateInfiltration(CreateInfiltration { index, id, zone_id, schedule_id, method, design_flow_ach, flow_per_exterior_area_m3_s_m2, effective_leakage_area_m2, discharge_coefficient, stack_height_m, constant_term_coefficient, temperature_term_coefficient, velocity_term_coefficient, velocity_squared_term_coefficient })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateInfiltration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "infiltration", kind: "create-infiltration", record: "CreatedInfiltration" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Infiltration {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
