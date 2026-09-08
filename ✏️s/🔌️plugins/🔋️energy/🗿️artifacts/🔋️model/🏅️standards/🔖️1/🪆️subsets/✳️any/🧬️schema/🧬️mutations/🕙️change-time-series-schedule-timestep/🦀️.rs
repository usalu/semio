//! 🕙️ Energy model mutation — `ChangeTimeSeriesScheduleTimestep`: Sets how many seconds one sample of a measured series covers — what turns its index into a clock.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕙️ `change-time-series-schedule-timestep` payload. Sets how many seconds one sample of a measured series covers — what turns its index into a clock.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-time-series-schedule-timestep")]
pub struct ChangeTimeSeriesScheduleTimestep {
    pub id: crate::model::ScheduleId,
    pub new_timestep_seconds: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_time_series_schedule_timestep(id: crate::model::ScheduleId, new_timestep_seconds: u32) -> EnergyModelMutation {
    EnergyModelMutation::ChangeTimeSeriesScheduleTimestep(ChangeTimeSeriesScheduleTimestep { id, new_timestep_seconds })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeTimeSeriesScheduleTimestep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "time-series-schedule", kind: "change-time-series-schedule-timestep", record: "ChangedTimeSeriesScheduleTimestep" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Time Series Schedule Timestep of time series schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
