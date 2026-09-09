//! 💠️ Energy model mutation — `ChangeMechanicalVentilationFanTotalEfficiency`: Sets fan total efficiency on one mechanical ventilation, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💠️ `change-mechanical-ventilation-fan-total-efficiency` payload. Sets fan total efficiency on one mechanical ventilation, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-fan-total-efficiency")]
pub struct ChangeMechanicalVentilationFanTotalEfficiency {
    pub id: crate::model::EntityId,
    pub new_fan_total_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_mechanical_ventilation_fan_total_efficiency(id: crate::model::EntityId, new_fan_total_efficiency: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMechanicalVentilationFanTotalEfficiency(ChangeMechanicalVentilationFanTotalEfficiency { id, new_fan_total_efficiency })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMechanicalVentilationFanTotalEfficiency {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "change", entity: "mechanical-ventilation", kind: "change-mechanical-ventilation-fan-total-efficiency", record: "ChangedMechanicalVentilationFanTotalEfficiency" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Mechanical Ventilation Fan Total Efficiency of mechanical ventilation {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
