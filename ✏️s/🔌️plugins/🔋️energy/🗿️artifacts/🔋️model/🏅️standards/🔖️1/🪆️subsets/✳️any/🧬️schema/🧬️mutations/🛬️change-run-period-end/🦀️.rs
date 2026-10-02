//! 🛬️ Energy model mutation — `ChangeRunPeriodEndMonth`: Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛬️ `change-run-period-end-month` payload. Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-period-end-month")]
pub struct ChangeRunPeriodEndMonth {
    pub new_end_month: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_period_end_month(new_end_month: u8) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunPeriodEndMonth(ChangeRunPeriodEndMonth { new_end_month })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunPeriodEndMonth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run-period", kind: "change-run-period-end-month", record: "ChangedRunPeriodEndMonth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period end month to {}", self.new_end_month), &format!("Endmonat des Simulationszeitraums auf {} ändern", self.new_end_month))
    }
}
//#endregion 🔖️Mutation
