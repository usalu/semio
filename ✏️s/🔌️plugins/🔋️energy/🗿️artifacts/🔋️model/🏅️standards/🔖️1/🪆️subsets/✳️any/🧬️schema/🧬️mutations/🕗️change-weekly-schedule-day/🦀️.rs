//! 🕗️ Energy model mutation — `ChangeWeeklyScheduleDay`: Points one day of a week at a different daily profile. The address is the pair (schedule id, day index 0–6); a day outside the week and a profile the document does not define are both refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕗️ `change-weekly-schedule-day` payload. Points one day of a week at a different daily profile. The address is the pair (schedule id, day index 0–6); a day outside the week and a profile the document does not define are both refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-weekly-schedule-day")]
pub struct ChangeWeeklyScheduleDay {
    pub id: crate::model::ScheduleId,
    pub day_index: u8,
    pub new_daily_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_weekly_schedule_day(id: crate::model::ScheduleId, day_index: u8, new_daily_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeWeeklyScheduleDay(ChangeWeeklyScheduleDay { id, day_index, new_daily_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeWeeklyScheduleDay {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "weekly-schedule", kind: "change-weekly-schedule-day", record: "ChangedWeeklyScheduleDay" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change weekly schedule {} day {}", self.id.0, self.day_index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
