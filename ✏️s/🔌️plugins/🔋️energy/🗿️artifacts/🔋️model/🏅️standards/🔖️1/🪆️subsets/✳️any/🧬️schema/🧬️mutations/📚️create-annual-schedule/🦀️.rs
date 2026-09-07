//! 📚️ Energy model mutation — `CreateAnnualSchedule`: Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📚️ `create-annual-schedule` payload. Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-annual-schedule")]
pub struct CreateAnnualSchedule {
    pub index: u32,
    pub id: crate::model::ScheduleId,
    pub default_daily_schedule_id: crate::model::ScheduleId,
    pub holiday_daily_schedule_id: Option<crate::model::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_annual_schedule(index: u32, id: crate::model::ScheduleId, default_daily_schedule_id: crate::model::ScheduleId, holiday_daily_schedule_id: Option<crate::model::ScheduleId>) -> EnergyModelMutation {
    EnergyModelMutation::CreateAnnualSchedule(CreateAnnualSchedule { index, id, default_daily_schedule_id, holiday_daily_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateAnnualSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "annual-schedule", kind: "create-annual-schedule", record: "CreatedAnnualSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Annual Schedule {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
