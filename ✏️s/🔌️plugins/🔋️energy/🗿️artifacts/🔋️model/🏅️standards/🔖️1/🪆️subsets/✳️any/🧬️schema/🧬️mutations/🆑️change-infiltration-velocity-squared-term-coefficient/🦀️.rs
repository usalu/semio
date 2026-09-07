//! 🆑️ Energy model mutation — `ChangeInfiltrationVelocitySquaredTermCoefficient`: Sets the squared wind velocity term coefficient D on one infiltration, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🆑️ `change-infiltration-velocity-squared-term-coefficient` payload. Sets the squared wind velocity term coefficient D on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-velocity-squared-term-coefficient")]
pub struct ChangeInfiltrationVelocitySquaredTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_velocity_squared_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_velocity_squared_term_coefficient(id: crate::model::EntityId, new_velocity_squared_term_coefficient: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationVelocitySquaredTermCoefficient(ChangeInfiltrationVelocitySquaredTermCoefficient { id, new_velocity_squared_term_coefficient })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationVelocitySquaredTermCoefficient {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-velocity-squared-term-coefficient", record: "ChangedInfiltrationVelocitySquaredTermCoefficient" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Velocity Squared Term Coefficient of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
