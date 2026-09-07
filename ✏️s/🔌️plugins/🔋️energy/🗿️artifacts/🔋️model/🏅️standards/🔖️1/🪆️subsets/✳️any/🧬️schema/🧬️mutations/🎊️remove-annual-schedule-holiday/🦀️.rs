//! 🎊️ Energy model mutation — `RemoveAnnualScheduleHoliday`: Takes one calendar date back out of a year's holiday set, so lookups on it fall back to the matching rule again.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎊️ `remove-annual-schedule-holiday` payload. Takes one calendar date back out of a year's holiday set, so lookups on it fall back to the matching rule again.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-annual-schedule-holiday")]
pub struct RemoveAnnualScheduleHoliday {
    pub id: crate::model::ScheduleId,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_annual_schedule_holiday(id: crate::model::ScheduleId, year: u16, month: u8, day: u8) -> EnergyModelMutation {
    EnergyModelMutation::RemoveAnnualScheduleHoliday(RemoveAnnualScheduleHoliday { id, year, month, day })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveAnnualScheduleHoliday {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "annual-schedule", kind: "remove-annual-schedule-holiday", record: "RemovedAnnualScheduleHoliday" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove holiday {}-{}-{} from annual schedule {}", self.year, self.month, self.day, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
