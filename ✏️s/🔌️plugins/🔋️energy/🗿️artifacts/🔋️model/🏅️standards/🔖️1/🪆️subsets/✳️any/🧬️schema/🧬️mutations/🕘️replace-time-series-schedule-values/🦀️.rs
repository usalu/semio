//! 🕘️ Energy model mutation — `ReplaceTimeSeriesScheduleValues`: Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕘️ `replace-time-series-schedule-values` payload. Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-time-series-schedule-values")]
pub struct ReplaceTimeSeriesScheduleValues {
    pub id: crate::model::ScheduleId,
    pub new_values: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_time_series_schedule_values(id: crate::model::ScheduleId, new_values: Vec<f64>) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceTimeSeriesScheduleValues(ReplaceTimeSeriesScheduleValues { id, new_values })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceTimeSeriesScheduleValues {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "time-series-schedule", kind: "replace-time-series-schedule-values", record: "ReplacedTimeSeriesScheduleValues" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace Time Series Schedule Values of time series schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
