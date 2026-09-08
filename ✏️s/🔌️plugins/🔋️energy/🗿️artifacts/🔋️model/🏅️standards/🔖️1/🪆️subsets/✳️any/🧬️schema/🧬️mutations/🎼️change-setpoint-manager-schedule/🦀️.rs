//! 🎼️ Energy model mutation — `ChangeSetpointManagerSchedule`: Points one setpoint manager at a schedule the model defines, or clears the slot. `Option<T>` has no `dsl::DslField`, so the optional reference travels as a `present` flag beside the id and an absent slot has to carry zero.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎼️ `change-setpoint-manager-schedule` payload. Points one setpoint manager at a schedule the model defines, or clears the slot. `Option<T>` has no `dsl::DslField`, so the optional reference travels as a `present` flag beside the id and an absent slot has to carry zero.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-setpoint-manager-schedule")]
pub struct ChangeSetpointManagerSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_present: bool,
    pub new_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_setpoint_manager_schedule(id: crate::model::EntityId, new_schedule_present: bool, new_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSetpointManagerSchedule(ChangeSetpointManagerSchedule { id, new_schedule_present, new_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSetpointManagerSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "setpoint-manager", kind: "change-setpoint-manager-schedule", record: "ChangedSetpointManagerSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change setpoint manager {} schedule to {:?}", self.id.0, self.new_schedule_present.then_some(self.new_schedule_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
