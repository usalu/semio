//! 🎄️ Energy model mutation — `ChangeAnnualScheduleHolidayDailySchedule`: Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎄️ `change-annual-schedule-holiday-daily-schedule` payload. Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-annual-schedule-holiday-daily-schedule")]
pub struct ChangeAnnualScheduleHolidayDailySchedule {
    pub id: crate::model::ScheduleId,
    pub new_holiday_daily_schedule_id: Option<crate::model::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_annual_schedule_holiday_daily_schedule(id: crate::model::ScheduleId, new_holiday_daily_schedule_id: Option<crate::model::ScheduleId>) -> EnergyModelMutation {
    EnergyModelMutation::ChangeAnnualScheduleHolidayDailySchedule(ChangeAnnualScheduleHolidayDailySchedule { id, new_holiday_daily_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeAnnualScheduleHolidayDailySchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annual-schedule", kind: "change-annual-schedule-holiday-daily-schedule", record: "ChangedAnnualScheduleHolidayDailySchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Annual Schedule Holiday Daily Schedule of annual schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
