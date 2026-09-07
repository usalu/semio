//! 🕞️ Energy model mutation — `CreateDailySchedule`: Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕞️ `create-daily-schedule` payload. Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-daily-schedule")]
pub struct CreateDailySchedule {
    pub index: u32,
    pub id: crate::model::ScheduleId,
    pub hourly_values: Vec<f64>,
    pub interpolation: crate::schedule::ScheduleInterpolation,
    pub limits_min: Option<f64>,
    pub limits_max: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_daily_schedule(index: u32, id: crate::model::ScheduleId, hourly_values: Vec<f64>, interpolation: crate::schedule::ScheduleInterpolation, limits_min: Option<f64>, limits_max: Option<f64>) -> EnergyModelMutation {
    EnergyModelMutation::CreateDailySchedule(CreateDailySchedule { index, id, hourly_values, interpolation, limits_min, limits_max })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateDailySchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "daily-schedule", kind: "create-daily-schedule", record: "CreatedDailySchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Daily Schedule {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
