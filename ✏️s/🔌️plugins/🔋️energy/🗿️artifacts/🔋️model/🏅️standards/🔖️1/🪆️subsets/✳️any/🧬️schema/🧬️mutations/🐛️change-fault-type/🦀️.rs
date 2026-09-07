//! 🐛️ Energy model mutation — `ChangeFaultType`: Sets one fault's degradation mechanism.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐛️ `change-fault-type` payload. Sets one fault's degradation mechanism.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-type")]
pub struct ChangeFaultType {
    pub id: crate::model::EntityId,
    pub new_fault_type: crate::model::FaultType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fault_type(id: crate::model::EntityId, new_fault_type: crate::model::FaultType) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFaultType(ChangeFaultType { id, new_fault_type })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFaultType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fault", kind: "change-fault-type", record: "ChangedFaultType" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Fault Type of fault {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
