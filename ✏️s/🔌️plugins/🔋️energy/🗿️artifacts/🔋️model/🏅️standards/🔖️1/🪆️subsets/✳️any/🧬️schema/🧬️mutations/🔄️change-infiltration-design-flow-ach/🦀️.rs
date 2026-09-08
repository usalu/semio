//! 🔄️ Energy model mutation — `ChangeInfiltrationDesignFlowAch`: Sets design flow (air changes per hour) on one infiltration, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔄️ `change-infiltration-design-flow-ach` payload. Sets design flow (air changes per hour) on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-design-flow-ach")]
pub struct ChangeInfiltrationDesignFlowAch {
    pub id: crate::model::EntityId,
    pub new_design_flow_ach: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_design_flow_ach(id: crate::model::EntityId, new_design_flow_ach: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationDesignFlowAch(ChangeInfiltrationDesignFlowAch { id, new_design_flow_ach })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationDesignFlowAch {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-design-flow-ach", record: "ChangedInfiltrationDesignFlowAch" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Design Flow Ach of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
