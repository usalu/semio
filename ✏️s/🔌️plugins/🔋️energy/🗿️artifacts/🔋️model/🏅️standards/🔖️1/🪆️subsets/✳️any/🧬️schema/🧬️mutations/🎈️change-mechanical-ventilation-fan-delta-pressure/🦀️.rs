//! 🎈️ Energy model mutation — `ChangeMechanicalVentilationFanDeltaPressure`: Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎈️ `change-mechanical-ventilation-fan-delta-pressure` payload. Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-fan-delta-pressure")]
pub struct ChangeMechanicalVentilationFanDeltaPressure {
    pub id: crate::model::EntityId,
    pub new_fan_delta_pressure_pa: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_mechanical_ventilation_fan_delta_pressure(id: crate::model::EntityId, new_fan_delta_pressure_pa: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMechanicalVentilationFanDeltaPressure(ChangeMechanicalVentilationFanDeltaPressure { id, new_fan_delta_pressure_pa })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMechanicalVentilationFanDeltaPressure {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mechanical-ventilation", kind: "change-mechanical-ventilation-fan-delta-pressure", record: "ChangedMechanicalVentilationFanDeltaPressure" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Mechanical Ventilation Fan Delta Pressure of mechanical ventilation {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
