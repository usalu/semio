//! ⏰️ Energy model mutation — `ChangePeopleGainSchedule`: Re-points one people gain's schedule reference; a schedule the document does not define is refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏰️ `change-people-gain-schedule` payload. Re-points one people gain's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-schedule")]
pub struct ChangePeopleGainSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_people_gain_schedule(id: crate::model::EntityId, new_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangePeopleGainSchedule(ChangePeopleGainSchedule { id, new_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePeopleGainSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-schedule", record: "ChangedPeopleGainSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change People Gain Schedule of people gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
