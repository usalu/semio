//! 🪗️ Energy model mutation — `CreateTimeSeriesSchedule`: Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪗️ `create-time-series-schedule` payload. Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-time-series-schedule")]
pub struct CreateTimeSeriesSchedule {
    pub index: u32,
    pub id: crate::model::ScheduleId,
    pub values: Vec<f64>,
    pub timestep_seconds: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_time_series_schedule(index: u32, id: crate::model::ScheduleId, values: Vec<f64>, timestep_seconds: u32) -> EnergyModelMutation {
    EnergyModelMutation::CreateTimeSeriesSchedule(CreateTimeSeriesSchedule { index, id, values, timestep_seconds })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateTimeSeriesSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "time-series-schedule", kind: "create-time-series-schedule", record: "CreatedTimeSeriesSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Time Series Schedule {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
