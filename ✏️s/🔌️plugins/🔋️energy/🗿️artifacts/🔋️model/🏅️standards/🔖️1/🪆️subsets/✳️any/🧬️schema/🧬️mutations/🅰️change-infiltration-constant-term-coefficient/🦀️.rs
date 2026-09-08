//! 🅰️ Energy model mutation — `ChangeInfiltrationConstantTermCoefficient`: Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0).

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🅰️ `change-infiltration-constant-term-coefficient` payload. Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-constant-term-coefficient")]
pub struct ChangeInfiltrationConstantTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_constant_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_constant_term_coefficient(id: crate::model::EntityId, new_constant_term_coefficient: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationConstantTermCoefficient(ChangeInfiltrationConstantTermCoefficient { id, new_constant_term_coefficient })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationConstantTermCoefficient {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-constant-term-coefficient", record: "ChangedInfiltrationConstantTermCoefficient" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Constant Term Coefficient of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
