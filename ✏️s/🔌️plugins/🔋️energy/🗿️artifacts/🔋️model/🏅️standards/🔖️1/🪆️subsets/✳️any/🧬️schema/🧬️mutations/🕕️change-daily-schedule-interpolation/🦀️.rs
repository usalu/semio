//! 🕕️ Energy model mutation — `ChangeDailyScheduleInterpolation`: Sets whether a daily profile steps between its hourly values or ramps across them.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕕️ `change-daily-schedule-interpolation` payload. Sets whether a daily profile steps between its hourly values or ramps across them.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daily-schedule-interpolation")]
pub struct ChangeDailyScheduleInterpolation {
    pub id: crate::model::ScheduleId,
    pub new_interpolation: crate::schedule::ScheduleInterpolation,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_daily_schedule_interpolation(id: crate::model::ScheduleId, new_interpolation: crate::schedule::ScheduleInterpolation) -> EnergyModelMutation {
    EnergyModelMutation::ChangeDailyScheduleInterpolation(ChangeDailyScheduleInterpolation { id, new_interpolation })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeDailyScheduleInterpolation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daily-schedule", kind: "change-daily-schedule-interpolation", record: "ChangedDailyScheduleInterpolation" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Daily Schedule Interpolation of daily schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
