//! ▶️ Energy model mutation — `ChangeRunStartDay`: Sets the day of month the simulation run period starts on; the period must stay a calendar interval of its year.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ▶️ `change-run-start-day` payload. Sets the day of month the simulation run period starts on; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-start-day")]
pub struct ChangeRunStartDay {
    pub new_start_day: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_run_start_day(new_start_day: u8) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRunStartDay(ChangeRunStartDay { new_start_day })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRunStartDay {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "run", kind: "change-run-start-day", record: "ChangedRunStartDay" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change run period start day to {}", self.new_start_day), &format!("Starttag des Simulationszeitraums auf {} ändern", self.new_start_day))
    }
}
//#endregion 🔖️Mutation
