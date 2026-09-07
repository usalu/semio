//! 🕔️ Energy model mutation — `ReplaceDailyScheduleHourlyValues`: Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕔️ `replace-daily-schedule-hourly-values` payload. Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-daily-schedule-hourly-values")]
pub struct ReplaceDailyScheduleHourlyValues {
    pub id: crate::model::ScheduleId,
    pub new_hourly_values: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_daily_schedule_hourly_values(id: crate::model::ScheduleId, new_hourly_values: Vec<f64>) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceDailyScheduleHourlyValues(ReplaceDailyScheduleHourlyValues { id, new_hourly_values })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceDailyScheduleHourlyValues {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "daily-schedule", kind: "replace-daily-schedule-hourly-values", record: "ReplacedDailyScheduleHourlyValues" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace Daily Schedule Hourly Values of daily schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
