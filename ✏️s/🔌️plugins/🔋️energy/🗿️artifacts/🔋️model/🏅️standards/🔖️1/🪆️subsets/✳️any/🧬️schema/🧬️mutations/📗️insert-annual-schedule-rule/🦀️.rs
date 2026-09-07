//! 📗️ Energy model mutation — `InsertAnnualScheduleRule`: Places one date rule at a stated position in a year's ordered rule list. The order is load-bearing, not cosmetic — `ScheduleSet::annual_value` returns the FIRST rule whose date range contains the day — so this is `insert` with a FINAL-state index, not a set-like `add`.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📗️ `insert-annual-schedule-rule` payload. Places one date rule at a stated position in a year's ordered rule list. The order is load-bearing, not cosmetic — `ScheduleSet::annual_value` returns the FIRST rule whose date range contains the day — so this is `insert` with a FINAL-state index, not a set-like `add`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "insert-annual-schedule-rule")]
pub struct InsertAnnualScheduleRule {
    pub id: crate::model::ScheduleId,
    pub index: u32,
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub daily_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn insert_annual_schedule_rule(id: crate::model::ScheduleId, index: u32, start_month: u8, start_day: u8, end_month: u8, end_day: u8, daily_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::InsertAnnualScheduleRule(InsertAnnualScheduleRule { id, index, start_month, start_day, end_month, end_day, daily_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for InsertAnnualScheduleRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "annual-schedule", kind: "insert-annual-schedule-rule", record: "InsertedAnnualScheduleRule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Insert rule at {} of annual schedule {}", self.index, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
