//! 🌪️ Energy model mutation — `CreateMechanicalVentilation`: Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌪️ `create-mechanical-ventilation` payload. Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-mechanical-ventilation")]
pub struct CreateMechanicalVentilation {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::model::ScheduleId,
    pub design_flow_m3_s: f64,
    pub fan_total_efficiency: f64,
    pub fan_delta_pressure_pa: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_mechanical_ventilation(
    index: u32,
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    schedule_id: crate::model::ScheduleId,
    design_flow_m3_s: f64,
    fan_total_efficiency: f64,
    fan_delta_pressure_pa: f64,
) -> EnergyModelMutation {
    EnergyModelMutation::CreateMechanicalVentilation(CreateMechanicalVentilation { index, id, zone_id, schedule_id, design_flow_m3_s, fan_total_efficiency, fan_delta_pressure_pa })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateMechanicalVentilation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mechanical-ventilation", kind: "create-mechanical-ventilation", record: "CreatedMechanicalVentilation" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Mechanical Ventilation {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
