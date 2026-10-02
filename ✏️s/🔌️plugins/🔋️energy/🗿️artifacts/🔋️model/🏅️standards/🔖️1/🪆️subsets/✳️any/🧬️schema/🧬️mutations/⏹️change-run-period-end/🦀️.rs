//! ⏹️ Energy model mutation — `ChangeRunPeriodEndDay`: Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏹️ `change-run-period-end-day` payload. Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-period-end-day")]
pub struct ChangeRunPeriodEndDay {
    pub new_end_day: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_period_end_day(new_end_day: u8) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunPeriodEndDay(ChangeRunPeriodEndDay { new_end_day })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunPeriodEndDay {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run-period", kind: "change-run-period-end-day", record: "ChangedRunPeriodEndDay" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period end day to {}", self.new_end_day), &format!("Endtag des Simulationszeitraums auf {} ändern", self.new_end_day))
    }
}
//#endregion 🔖️Mutation
