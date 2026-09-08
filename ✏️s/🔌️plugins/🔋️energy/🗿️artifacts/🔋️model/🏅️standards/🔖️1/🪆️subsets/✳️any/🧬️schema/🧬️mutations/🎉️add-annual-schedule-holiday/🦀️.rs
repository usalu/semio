//! 🎉️ Energy model mutation — `AddAnnualScheduleHoliday`: Marks one calendar date as a holiday of a year, so lookups on it take the holiday profile instead of the matching rule. The dates are a set, but a JSON array positionally, so the payload carries the position the date takes.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎉️ `add-annual-schedule-holiday` payload. Marks one calendar date as a holiday of a year, so lookups on it take the holiday profile instead of the matching rule. The dates are a set, but a JSON array positionally, so the payload carries the position the date takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-annual-schedule-holiday")]
pub struct AddAnnualScheduleHoliday {
    pub id: crate::model::ScheduleId,
    pub index: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_annual_schedule_holiday(id: crate::model::ScheduleId, index: u32, year: u16, month: u8, day: u8) -> EnergyModelMutation {
    EnergyModelMutation::AddAnnualScheduleHoliday(AddAnnualScheduleHoliday { id, index, year, month, day })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddAnnualScheduleHoliday {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "annual-schedule", kind: "add-annual-schedule-holiday", record: "AddedAnnualScheduleHoliday" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add holiday {}-{}-{} to annual schedule {}", self.year, self.month, self.day, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
