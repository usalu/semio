//! 🌫️ Energy model mutation — `ChangeInfiltrationFlowPerExteriorArea`: Sets flow per exterior area (m³/s·m²) on one infiltration, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌫️ `change-infiltration-flow-per-exterior-area` payload. Sets flow per exterior area (m³/s·m²) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-flow-per-exterior-area")]
pub struct ChangeInfiltrationFlowPerExteriorArea {
    pub id: crate::model::EntityId,
    pub new_flow_per_exterior_area_m3_s_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_flow_per_exterior_area(id: crate::model::EntityId, new_flow_per_exterior_area_m3_s_m2: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationFlowPerExteriorArea(ChangeInfiltrationFlowPerExteriorArea { id, new_flow_per_exterior_area_m3_s_m2 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationFlowPerExteriorArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-flow-per-exterior-area", record: "ChangedInfiltrationFlowPerExteriorArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Flow Per Exterior Area of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
