//! 📅️ Energy model mutation — `ChangeRunPeriodYear`: Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📅️ `change-run-period-year` payload. Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-period-year")]
pub struct ChangeRunPeriodYear {
    pub new_year: u16,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_period_year(new_year: u16) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunPeriodYear(ChangeRunPeriodYear { new_year })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunPeriodYear {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run-period", kind: "change-run-period-year", record: "ChangedRunPeriodYear" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period year to {}", self.new_year), &format!("Jahr des Simulationszeitraums auf {} ändern", self.new_year))
    }
}
//#endregion 🔖️Mutation
