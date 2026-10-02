//! 🛫️ Energy model mutation — `ChangeRunPeriodStartMonth`: Sets the month the simulation run period starts in; the period must stay a calendar interval of its year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛫️ `change-run-period-start-month` payload. Sets the month the simulation run period starts in; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-period-start-month")]
pub struct ChangeRunPeriodStartMonth {
    pub new_start_month: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_period_start_month(new_start_month: u8) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunPeriodStartMonth(ChangeRunPeriodStartMonth { new_start_month })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunPeriodStartMonth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run-period", kind: "change-run-period-start-month", record: "ChangedRunPeriodStartMonth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period start month to {}", self.new_start_month), &format!("Startmonat des Simulationszeitraums auf {} ändern", self.new_start_month))
    }
}
//#endregion 🔖️Mutation
