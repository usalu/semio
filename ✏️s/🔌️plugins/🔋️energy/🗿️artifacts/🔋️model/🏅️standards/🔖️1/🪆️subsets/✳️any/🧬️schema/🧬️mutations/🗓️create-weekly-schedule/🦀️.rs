//! 🗓️ Energy model mutation — `CreateWeeklySchedule`: Defines one week as seven daily profiles, Sunday first — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗓️ `create-weekly-schedule` payload. Defines one week as seven daily profiles, Sunday first — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-weekly-schedule")]
pub struct CreateWeeklySchedule {
    pub index: u32,
    pub id: crate::model::ScheduleId,
    pub daily_schedule_ids: Vec<crate::model::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_weekly_schedule(index: u32, id: crate::model::ScheduleId, daily_schedule_ids: Vec<crate::model::ScheduleId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateWeeklySchedule(CreateWeeklySchedule { index, id, daily_schedule_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateWeeklySchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "weekly-schedule", kind: "create-weekly-schedule", record: "CreatedWeeklySchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Weekly Schedule {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
