//! 🚰️ Energy model mutation — `ChangeInfiltrationDischargeCoefficient`: Sets the orifice discharge coefficient on one infiltration, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚰️ `change-infiltration-discharge-coefficient` payload. Sets the orifice discharge coefficient on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-discharge-coefficient")]
pub struct ChangeInfiltrationDischargeCoefficient {
    pub id: crate::model::EntityId,
    pub new_discharge_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_discharge_coefficient(id: crate::model::EntityId, new_discharge_coefficient: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationDischargeCoefficient(ChangeInfiltrationDischargeCoefficient { id, new_discharge_coefficient })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationDischargeCoefficient {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-discharge-coefficient", record: "ChangedInfiltrationDischargeCoefficient" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Discharge Coefficient of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
