//! 🕟️ Energy model mutation — `ChangeDailyScheduleLimits`: Sets the optional clamp a daily profile's lookups are bounded by. Lower and upper are one inseparable pair — a half-stated pair is refused — and stating neither clears the clamp entirely.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕟️ `change-daily-schedule-limits` payload. Sets the optional clamp a daily profile's lookups are bounded by. Lower and upper are one inseparable pair — a half-stated pair is refused — and stating neither clears the clamp entirely.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daily-schedule-limits")]
pub struct ChangeDailyScheduleLimits {
    pub id: crate::model::ScheduleId,
    pub new_limits_min: Option<f64>,
    pub new_limits_max: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_daily_schedule_limits(id: crate::model::ScheduleId, new_limits_min: Option<f64>, new_limits_max: Option<f64>) -> EnergyModelMutation {
    EnergyModelMutation::ChangeDailyScheduleLimits(ChangeDailyScheduleLimits { id, new_limits_min, new_limits_max })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeDailyScheduleLimits {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daily-schedule", kind: "change-daily-schedule-limits", record: "ChangedDailyScheduleLimits" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change daily schedule {} limits", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
