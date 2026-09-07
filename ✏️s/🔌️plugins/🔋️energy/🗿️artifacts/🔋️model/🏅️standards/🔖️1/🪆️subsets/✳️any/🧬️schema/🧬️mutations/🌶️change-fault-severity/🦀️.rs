//! 🌶️ Energy model mutation — `ChangeFaultSeverity`: Sets severity on one fault, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌶️ `change-fault-severity` payload. Sets severity on one fault, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-severity")]
pub struct ChangeFaultSeverity {
    pub id: crate::model::EntityId,
    pub new_severity: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fault_severity(id: crate::model::EntityId, new_severity: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFaultSeverity(ChangeFaultSeverity { id, new_severity })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFaultSeverity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fault", kind: "change-fault-severity", record: "ChangedFaultSeverity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Fault Severity of fault {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
