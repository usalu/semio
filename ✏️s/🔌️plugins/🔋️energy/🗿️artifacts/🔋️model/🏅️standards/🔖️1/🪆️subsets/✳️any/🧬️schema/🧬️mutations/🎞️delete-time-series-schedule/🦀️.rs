//! 🎞️ Energy model mutation — `DeleteTimeSeriesSchedule`: Removes one measured series. Refused while any consumer still resolves its id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎞️ `delete-time-series-schedule` payload. Removes one measured series. Refused while any consumer still resolves its id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-time-series-schedule")]
pub struct DeleteTimeSeriesSchedule {
    pub id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_time_series_schedule(id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteTimeSeriesSchedule(DeleteTimeSeriesSchedule { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteTimeSeriesSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "time-series-schedule", kind: "delete-time-series-schedule", record: "DeletedTimeSeriesSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Time Series Schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
