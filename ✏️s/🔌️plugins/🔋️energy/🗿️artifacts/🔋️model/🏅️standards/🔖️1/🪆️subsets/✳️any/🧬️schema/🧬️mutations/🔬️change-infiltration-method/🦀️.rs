//! 🔬️ Energy model mutation — `ChangeInfiltrationMethod`: Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔬️ `change-infiltration-method` payload. Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-method")]
pub struct ChangeInfiltrationMethod {
    pub id: crate::model::EntityId,
    pub new_method: crate::air_exchange::InfiltrationMethod,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_infiltration_method(id: crate::model::EntityId, new_method: crate::air_exchange::InfiltrationMethod) -> EnergyModelMutation {
    EnergyModelMutation::ChangeInfiltrationMethod(ChangeInfiltrationMethod { id, new_method })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeInfiltrationMethod {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration", kind: "change-infiltration-method", record: "ChangedInfiltrationMethod" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Infiltration Method of infiltration {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
