//! 📅️ Energy model mutation — `UpdateRunPeriod`: Sets the whole inseparable run-period facet the simulation kernel reads out of the model. Start and end are one calendar interval: setting either alone can name an interval that does not exist.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📅️ `update-run-period` payload. Sets the whole inseparable run-period facet the simulation kernel reads out of the model. Start and end are one calendar interval: setting either alone can name an interval that does not exist.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "update-run-period")]
pub struct UpdateRunPeriod {
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub year: u16,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_run_period(start_month: u8, start_day: u8, end_month: u8, end_day: u8, year: u16) -> EnergyModelMutation {
    EnergyModelMutation::UpdateRunPeriod(UpdateRunPeriod { start_month, start_day, end_month, end_day, year })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for UpdateRunPeriod {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "run-period", kind: "update-run-period", record: "UpdatedRunPeriod" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Update run period to {}-{} .. {}-{}", self.start_month, self.start_day, self.end_month, self.end_day)
    }
}
//#endregion 🔖️Mutation
