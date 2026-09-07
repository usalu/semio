//! 🕓️ Energy model mutation — `ChangeFaultStartSchedule`: Re-points one fault's schedule reference; a schedule the document does not define is refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕓️ `change-fault-start-schedule` payload. Re-points one fault's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-start-schedule")]
pub struct ChangeFaultStartSchedule {
    pub id: crate::model::EntityId,
    pub new_start_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fault_start_schedule(id: crate::model::EntityId, new_start_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFaultStartSchedule(ChangeFaultStartSchedule { id, new_start_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFaultStartSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fault", kind: "change-fault-start-schedule", record: "ChangedFaultStartSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Fault Start Schedule of fault {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
