//! 🎌️ Energy model mutation — `ChangeAnnualScheduleDefaultDailySchedule`: Re-points the profile a year falls back to on every day no rule matches.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎌️ `change-annual-schedule-default-daily-schedule` payload. Re-points the profile a year falls back to on every day no rule matches.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-annual-schedule-default-daily-schedule")]
pub struct ChangeAnnualScheduleDefaultDailySchedule {
    pub id: crate::model::ScheduleId,
    pub new_default_daily_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_annual_schedule_default_daily_schedule(id: crate::model::ScheduleId, new_default_daily_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeAnnualScheduleDefaultDailySchedule(ChangeAnnualScheduleDefaultDailySchedule { id, new_default_daily_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeAnnualScheduleDefaultDailySchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annual-schedule", kind: "change-annual-schedule-default-daily-schedule", record: "ChangedAnnualScheduleDefaultDailySchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Annual Schedule Default Daily Schedule of annual schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
