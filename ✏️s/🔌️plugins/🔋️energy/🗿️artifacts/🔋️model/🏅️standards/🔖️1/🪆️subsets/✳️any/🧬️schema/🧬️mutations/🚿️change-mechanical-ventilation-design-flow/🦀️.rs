//! 🚿️ Energy model mutation — `ChangeMechanicalVentilationDesignFlow`: Sets design supply flow (m³/s) on one mechanical ventilation, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚿️ `change-mechanical-ventilation-design-flow` payload. Sets design supply flow (m³/s) on one mechanical ventilation, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-design-flow")]
pub struct ChangeMechanicalVentilationDesignFlow {
    pub id: crate::model::EntityId,
    pub new_design_flow_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_mechanical_ventilation_design_flow(id: crate::model::EntityId, new_design_flow_m3_s: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeMechanicalVentilationDesignFlow(ChangeMechanicalVentilationDesignFlow { id, new_design_flow_m3_s })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeMechanicalVentilationDesignFlow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mechanical-ventilation", kind: "change-mechanical-ventilation-design-flow", record: "ChangedMechanicalVentilationDesignFlow" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Mechanical Ventilation Design Flow of mechanical ventilation {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
